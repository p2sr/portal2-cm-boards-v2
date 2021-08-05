use actix_web::{get, post, web, HttpResponse, Error};
use std::collections::HashMap;

use crate::db::DbPool;
use crate::tools::datamodels::{SPMap, SpPbHistory, SpPreviews, SpScoreParams, SPRanked, SpBanned, Changelog, Usersnew};
use crate::tools::calc::score;

/// Endpoint to handle the preview page showing all sp maps.
/// Returns: Json wrapped values -> { map_name, scores{ map_id (steam_id), profile_number, score, youtube_id, category, boardname, steamname } }
#[get("/sp")]
async fn get_singleplayer_preview(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
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

/// Calls models::SPMap to grab the entries for a particular mapid, returns a vector of the top 200 times, in a slimmed down fashion (only essential data)
/// Handles filtering out obsolete times (1 time per runner)
#[get("/maps/sp/{mapid}")]
async fn get_singleplayer_maps(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
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
/// Gives the profile number and score for all banned times on a given SP map
#[get("/maps/sp/banned/{mapid}")]
async fn get_banned_scores(mapid: web::Path<u64>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let banned_entries = web::block(move || SpBanned::show(&conn, mapid.to_string()))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(banned_entries))
}


// TODO: Probably should still improve error handling, but now the call is web::block()ing
/// Gives the profile number and score for all banned times on a given SP map
#[post("/maps/sp/banned/{mapid}")]
async fn post_banned_scores(mapid: web::Path<u64>, params: web::Json<SpScoreParams>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let banned_entries = web::block(move || Changelog::check_banned_scores(&conn, mapid.to_string(), params.score, params.profilenumber.clone()))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        // HttpResponse::InternalServerError().finish()
    });
    match banned_entries{
        Ok(true) => return Ok(HttpResponse::Ok().json(true)),
        Ok(false) => return Ok(HttpResponse::Ok().json(false)),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Returns a players PB history on an SP map
#[get("/maps/sp/{mapid}/{profilenumber}")]
async fn get_sp_pbs(info: web::Path<(i32, i32)>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    
    // This is gross but Rust was being dumb so I had to do a bunch of weird working around.
    let new_info = info.0;
    let profile_number = new_info.0.to_string();
    let map_id = new_info.1.to_string();
    let map_id_copy = map_id.clone();

    // Get usersnew info for the player. It should be reusable.
    let user_data = web::block(move || Usersnew::show(&conn, map_id))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let changelog_data = web::block(move || Changelog::sp_pb_history(&conn, profile_number, map_id_copy))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(SpPbHistory {
        user_info: user_data.unwrap(),
        pb_history: changelog_data,
    }))
}

