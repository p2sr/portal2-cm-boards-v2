use crate::controllers::models::{
    Changelog, CoopBanned, CoopBundled, CoopBundledInsert, CoopMap, CoopPreviews, CoopRanked,
    ScoreParams,
};
use crate::tools::cache::{read_from_file, write_to_file, CacheState};
use crate::tools::calc::score;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;

/// Endpoint to handle the preview page showing all coop maps.
#[get("/coop")]
async fn get_cooperative_preview(
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    let state_data = &mut cache.current_state.lock().await;
    let is_cached = state_data.get_mut("coop_previews").unwrap();
    if !*is_cached {
        let res = CoopPreviews::get_coop_previews(pool.get_ref()).await;
        match res {
            Ok(previews) => {
                if write_to_file("coop_previews", &previews).await.is_ok() {
                    *is_cached = true;
                    HttpResponse::Ok().json(previews)
                } else {
                    eprintln!("Could not write cache for coop previews");
                    HttpResponse::Ok().json(previews)
                }
            }
            _ => HttpResponse::NotFound().body("Error fetching coop map previews."),
        }
    } else {
        let res = read_from_file::<Vec<CoopPreviews>>("coop_previews").await;
        match res {
            Ok(previews) => HttpResponse::Ok().json(previews),
            _ => HttpResponse::NotFound().body("Error fetching coop previews from cache"),
        }
    }
}

/// Handles filtering out obsolete times (1 per runner, allowed for more than 1 if a time is with a player without a better time)
#[get("/maps/coop/{map_id}")]
async fn get_cooperative_maps(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = CoopMap::get_coop_map_page(pool.get_ref(), map_id.to_string()).await;
    match res {
        Ok(coop_entries) => {
            //Filters out all obsolete times from the result, then truncates to 200 entries.
            let mut coop_entries_filtered = Vec::new();
            let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(500);
            let mut i = 1;
            remove_dups.insert("".to_string(), 1);
            for entry in coop_entries {
                match remove_dups.insert(entry.profile_number1.clone(), 1) {
                    // If player 1 has a better time, check to see if player 2 doesn't.
                    Some(_) => match remove_dups.insert(entry.profile_number2.clone(), 1) {
                        Some(_) => (),
                        _ => {
                            coop_entries_filtered.push(CoopRanked {
                                map_data: entry.clone(),
                                rank: i,
                                points: score(i),
                            });
                            i += 1;
                        }
                    },
                    // This case handles if player 1 doesn't have a better time, and it tries to add player 2 in as well, if two has a better time or not, this is included.
                    _ => match remove_dups.insert(entry.profile_number2.clone(), 1) {
                        Some(_) => {
                            coop_entries_filtered.push(CoopRanked {
                                map_data: entry.clone(),
                                rank: i,
                                points: score(i),
                            });
                            i += 1;
                        }
                        _ => {
                            coop_entries_filtered.push(CoopRanked {
                                map_data: entry.clone(),
                                rank: i,
                                points: score(i),
                            });
                            i += 1;
                        }
                    },
                }
            }
            coop_entries_filtered.truncate(200);
            HttpResponse::Ok().json(coop_entries_filtered)
        }
        _ => HttpResponse::NotFound().body("Error fetching Coop Map Page"),
    }
}

/// Returns two profile numbers and the score for all banned times on a coop map.
#[get("/maps/coop/banned/{map_id}")]
async fn get_banned_scores_coop(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = CoopBanned::get_coop_banned(pool.get_ref(), map_id.to_string()).await;
    match res {
        Ok(banned_entries) => HttpResponse::Ok().json(banned_entries),
        _ => HttpResponse::NotFound().body("Error fetching Coop banned information"),
    }
}

// TODO: Probably should still improve error handling, but now the call is web::block()ing
/// Gives the profile number and score for all banned times on a given Coop map. Same as SP for now
#[post("/maps/coop/banned/{map_id}")]
async fn post_banned_scores_coop(
    map_id: web::Path<u64>,
    params: web::Json<ScoreParams>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // Potentially check for a valid coop map_id before spawning a thread to query the database.
    let res = Changelog::check_banned_scores(
        pool.get_ref(),
        map_id.to_string(),
        params.score,
        params.profile_number.clone(),
    )
    .await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        Err(_) => HttpResponse::NotFound().body("Error checking ban information."),
    }
}

#[post("/coop/post_score")]
async fn post_score_coop(
    params: web::Json<CoopBundledInsert>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    #[allow(unused_variables)]
    let res = CoopBundled::insert_coop_bundled(pool.get_ref(), params.0).await;
    // match res {
    //     Ok(id) => HttpResponse::Ok().json(id),
    //     _ => HttpResponse::NotFound().body("Error adding new score to database."),
    // }
    let id = 1;
    HttpResponse::Ok().json(id)
}
