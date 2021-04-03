use actix_web::{get, body::Body, http::header, web, HttpResponse, Error};
use std::collections::HashMap;
use std::cmp::max;
use num::pow;

use crate::db::DbPool;
use crate::structs::Changelog;
use crate::structs::SPMap;
use crate::structs::CoopMap;
use crate::structs::SpPreviews;
use crate::structs::CoopPreviews;

// Calls models::Changelog::all with a connection from the pool tog grab the test
// The web::block() moves the function outside of a blocking context onto another worker thread
#[get("/test/sp/")]
async fn dbpool_test(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let cl = web::block(move || Changelog::all(&conn))
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(cl) = cl{
        Ok(HttpResponse::Ok().json(cl))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}
// Wrapper for the sp map data and the rank/score
#[derive(Serialize)]
pub struct SPRanked{
    pub map_data: SPMap,
    pub rank: i32,
    pub score: f32,
}
// Wrapper for the coop map data and the rank/score
#[derive(Serialize)]
pub struct CoopRanked{
    pub map_data: CoopMap,
    pub rank: i32,
    pub score: f32,
}
// Calcultes the score as a 32bit float from the rank (i)
pub fn score(i: i32) -> f32{
    let i = i as f32;
    let res: f32 = pow(200.0-(i-1.0), 2)/200.0;
    if 1.0 > res{
        return 1.0;
    } else{
        return res;
    }
}

// Calls models::SPMap to grab the entries for a particular mapid, returns a vector of the top 200 times, in a slimmed down fashion (only essential data)
// Handles filtering out obsolete times (1 time per runner)
#[get("/maps/sp/{mapid}")]
async fn singleplayer_maps(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    // Grabs a mysql db connection from a pool in the web::Data.
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    // Async non-blocking call to grab the data from the database.
    let changelog_entries = web::block(move || SPMap::show(&conn, mapid.to_string()))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    // Check the contents of the Option, if it exists extract it. 
    if let Some(changelog_entries) = changelog_entries{
        // Filters out all obsolete times from the result, then truncates to 200 entries.
        let mut changelog_entries_filtered = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(200);
        let mut i = 1;
        for entry in changelog_entries.iter(){
            match remove_dups.insert(entry.profile_number.clone(), 1){
                // If this returns, the profile_number has a better time, remove the time from the vector
                Some(_) => (),
                _ => {
                    changelog_entries_filtered.push( SPRanked {map_data: entry.clone(), rank: i, score: score(i)});
                    i += 1;
                }
            }
        }
        changelog_entries_filtered.truncate(200);
        // Return a response ok and serialize the vector into a JSON object.
        Ok(HttpResponse::Ok().json(changelog_entries_filtered))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

/// Endpoint to handle the preview page showing all sp maps.
    /// Returns: Json wrapped values -> { map_name, scores{ map_id (steam_id), profile_number, score, youtube_id, category, boardname, steamname } }
#[get("/sp")]
async fn singleplayer_preview(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let sp_previews = web::block(move || SpPreviews::show(&conn))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    // Check the contents of the Option, if it exists extract it. 
    if let Some(sp_previews) = sp_previews{
        Ok(HttpResponse::Ok().json(sp_previews))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

/// Endpoint to handle the preview page showing all coop maps.
    /// Returns: Json wrapped values -> { map_name, scores{ map_id (steam_id), profile_number (1 & 2), score, youtube_id (1 & 2), category, boardname (1 & 2), steamname (1 & 2)} }
#[get("/coop")]
async fn cooperative_preview(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let coop_previews = web::block(move || CoopPreviews::show(&conn))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    // Check the contents of the Option, if it exists extract it. 
    if let Some(coop_previews) = coop_previews{
        Ok(HttpResponse::Ok().json(coop_previews))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

/// Calls models::CoopMap to grab the entries for a particular mapid, returns a vector of the top 200 times, in a slimmed down fashion (only essential data)
/// Handles filtering out obsolete times (1 per runner, allowed for more than 1 if a time is with a player without a better time)
// TODO: Implement aliased queries (waiting on you diesel peer review team)
#[get("/maps/coop/{mapid}")]
async fn coop_maps(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let coopbundled_entries = web::block(move || CoopMap::show(&conn, mapid.to_string()))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if let Some(coopbundled_entries) = coopbundled_entries{
        // Filters out all obsolete times from the result, then truncates to 200 entries.
        let mut coopbundled_entries_filtered = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(500);
        let mut i = 1;
        remove_dups.insert("".to_string(), 1);
        for entry in coopbundled_entries{
            match remove_dups.insert(entry.profile_number1.clone(), 1){
                // If player 1 has a better time, check to see if player 2 doesn't.
                Some(_) => match remove_dups.insert(entry.profile_number2.clone(), 1){
                    Some(_) => (),
                    _ => {
                        coopbundled_entries_filtered.push(CoopRanked {map_data: entry.clone(), rank: i, score: score(i)});
                        i += 1;
                    }
                }
                // This case handles if player 1 doesn't have a better time, and it tries to add player 2 in as well, if two has a better time or not, this is included.
                _ => match remove_dups.insert(entry.profile_number2.clone(), 1){
                    Some(_) =>{
                         coopbundled_entries_filtered.push( CoopRanked {map_data: entry.clone(), rank: i, score: score(i)});
                         i += 1;
                    }
                    _ => {
                        coopbundled_entries_filtered.push(CoopRanked {map_data: entry.clone(), rank: i, score: score(i)});
                        i += 1;
                    }
                }    
            }
        }
        coopbundled_entries_filtered.truncate(200);
        Ok(HttpResponse::Ok().json(coopbundled_entries_filtered))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}


// Mounts the routes to /api/..
pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(singleplayer_maps)
            .service(coop_maps)
            .service(singleplayer_preview)
            .service(cooperative_preview)
            .service(dbpool_test)
    );
}