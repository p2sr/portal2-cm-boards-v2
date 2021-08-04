use actix_web::{get, web, HttpResponse, Error};
use std::collections::HashMap;

use crate::db::DbPool;
use crate::tools::datamodels::{CoopMap, CoopBanned, CoopPreviews, CoopRanked};
use crate::tools::calc::score;


/// Endpoint to handle the preview page showing all coop maps.
#[get("/coop")]
async fn get_cooperative_preview(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
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
// TODO: Implement aliased queries (waiting on you diesel peer review team)
/// Calls `models::CoopMap` to grab the entries for a particular mapid, returns a vector of the top 200 times, in a slimmed down fashion (only essential data)
/// Handles filtering out obsolete times (1 per runner, allowed for more than 1 if a time is with a player without a better time)
#[get("/maps/coop/{mapid}")]
async fn get_cooperative_maps(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
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
/// Returns two profile numbers and the score for all banned times on a coop map.
#[get("/maps/coop/banned/{mapid}")]
async fn get_banned_scores(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let banned_entries = web::block(move || CoopBanned::show(&conn, mapid.to_string()))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(banned_entries))
}