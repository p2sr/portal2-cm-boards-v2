use actix_web::{get, web, HttpResponse, Error};
use std::collections::HashMap;

use crate::db::DbPool;
use crate::tools::datamodels::{SPMap, SpPreviews, SPRanked};
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