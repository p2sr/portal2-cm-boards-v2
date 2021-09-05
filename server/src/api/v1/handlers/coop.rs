use actix_web::{get, post, web, HttpResponse, Responder};
use std::collections::HashMap;
use sqlx::PgPool;
use crate::tools::datamodels::{Changelog, ScoreParams, CoopMap, CoopBanned, CoopPreviews, CoopRanked};
use crate::tools::calc::score;


/// Endpoint to handle the preview page showing all coop maps.
#[get("/coop")]
async fn get_cooperative_preview(pool: web::Data<PgPool>) -> impl Responder {
    let res = CoopPreviews::get_coop_previews(pool.get_ref()).await;
    match res{
        Ok(previews) => HttpResponse::Ok().json(previews),
        _ => HttpResponse::NotFound().body("Error fetching coop map previews."),
    }
}

/// Handles filtering out obsolete times (1 per runner, allowed for more than 1 if a time is with a player without a better time)
#[get("/maps/coop/{map_id}")]
async fn get_cooperative_maps(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder{
    let res = CoopMap::get_coop_map_page(pool.get_ref(), map_id.to_string()).await;
    match res{
        Ok(coop_entries) => {
        //Filters out all obsolete times from the result, then truncates to 200 entries.
        let mut coop_entries_filtered  = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(500);
        let mut i = 1;
        remove_dups.insert("".to_string(), 1);
        for entry in coop_entries{
            match remove_dups.insert(entry.profile_number1.clone(), 1){
                // If player 1 has a better time, check to see if player 2 doesn't.
                Some(_) => match remove_dups.insert(entry.profile_number2.clone(), 1){
                    Some(_) => (),
                    _ => {
                        coop_entries_filtered.push(CoopRanked {
                            map_data: entry.clone(),
                            rank: i,
                            points: score(i)
                        });
                        i += 1;
                    }
                }
                // This case handles if player 1 doesn't have a better time, and it tries to add player 2 in as well, if two has a better time or not, this is included.
                _ => match remove_dups.insert(entry.profile_number2.clone(), 1){
                    Some(_) =>{
                        coop_entries_filtered.push( CoopRanked {
                            map_data: entry.clone(),
                            rank: i,
                            points: score(i)
                        });
                        i += 1;
                    }
                    _ => {
                        coop_entries_filtered.push(CoopRanked {
                            map_data: entry.clone(),
                            rank: i,
                            points: score(i)
                        });
                        i += 1;
                    }
                }    
            }
        }
        coop_entries_filtered.truncate(200);
        HttpResponse::Ok().json(coop_entries_filtered)
    },
        _ => HttpResponse::NotFound().body("Error fetching Coop Map Page"),
    }
}

/// Returns two profile numbers and the score for all banned times on a coop map.
#[get("/maps/coop/banned/{map_id}")]
async fn get_banned_scores(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder{
    let res = CoopBanned::get_coop_banned(pool.get_ref(), map_id.to_string()).await;
    match res{
        Ok(banned_entries) => HttpResponse::Ok().json(banned_entries),
        _ => HttpResponse::NotFound().body("Error fetching Coop banned information"),
    }
}

// TODO: Probably should still improve error handling, but now the call is web::block()ing
/// Gives the profile number and score for all banned times on a given Coop map. Same as SP for now
#[post("/maps/coop/banned/{map_id}")]
async fn post_banned_scores(map_id: web::Path<u64>, params: web::Json<ScoreParams>, pool: web::Data<PgPool>) -> impl Responder{
    // Potentially check for a valid coop map_id before spawning a thread to query the database.
    let res = Changelog::check_banned_scores(pool.get_ref(), map_id.to_string(), params.score, params.profile_number.clone()).await;
    match res{
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        Err(_) => HttpResponse::NotFound().body("Error checking ban information."),
    }
}

// TODO: How do we want to handle PB history for coop? Do we want a coop bundled for each player? Do we want just an SP changelog?
// For this particular instance, I think we want to have multiple endpoints. 
// #[get("/maps/coop/{map_id}/{profilenumber1}/{profilenumber2}")]
// async fn get_sp_pbs(info: web::Path<(i32, i32, i32)>, pool: web::Data<PgPool>) -> impl Responder{
//   
    
//     // This is gross but Rust was being dumb so I had to do a bunch of weird working around.
//     let new_info = info.0;
//     let map_id = new_info.0.to_string();
//     let profile_number1 = new_info.1.to_string();
//     let profile_number2 = new_info.2.to_string();
//     let map_id_copy = map_id.clone();

//     // Get usersnew info for the player. It should be reusable.
//     let user_data = web::block(move || Usersnew::show(pool.get_ref(), map_id))
//         .await
//         .map_err(|e|{
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;
//   
//     let changelog_data = web::block(move || Changelog::sp_pb_history(pool.get_ref(), profile_number, map_id_copy))
//         .await
//         .map_err(|e|{
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;
//     if let Some(changelog_data) = changelog_data{
//         Ok(HttpResponse::Ok().json(SpPbHistory {
//             user_info: user_data.unwrap(),
//             pb_history: Some(changelog_data),
//         }))
//     } else{
//         Ok(HttpResponse::Ok().json(SpPbHistory {
//             user_info: user_data.unwrap(),
//             pb_history: None,
//         }))
//     }
// }