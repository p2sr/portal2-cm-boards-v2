use crate::controllers::models::{
    Changelog, ChangelogInsert, Opti32, ScoreParams, SpBanned, SpMap, SpPbHistory, SpPreviews,
    SpRanked, Users, UsersPage,
};
use crate::tools::cache::{read_from_file, write_to_file, CacheState};
use crate::tools::helpers::check_for_valid_score;
use crate::tools::{config::Config, helpers::score};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// GET endpoint to handle the preview page showing all sp maps.
/// Returns: Json wrapped values -> {map_id, scores{ steam_id, profile_number, score, youtube_id, category_id, user_name } }
#[get("/sp")]
async fn get_singleplayer_preview(
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    // See if we can utilize the cache
    let state_data = &mut cache.current_state.lock().await;
    let is_cached = state_data.get_mut("sp_previews").unwrap();
    if !*is_cached {
        let res = SpPreviews::get_sp_previews(pool.get_ref()).await;
        match res {
            Ok(sp_previews) => {
                if write_to_file("sp_previews", &sp_previews).await.is_ok() {
                    *is_cached = true;
                    HttpResponse::Ok().json(sp_previews)
                } else {
                    eprintln!("Could not write cache for sp previews");
                    HttpResponse::Ok().json(sp_previews)
                }
            }
            _ => HttpResponse::NotFound().body("Error fetching previews"),
        }
    } else {
        let res = read_from_file::<Vec<SpPreviews>>("sp_previews").await;
        match res {
            Ok(sp_previews) => HttpResponse::Ok().json(sp_previews),
            _ => HttpResponse::NotFound().body("Error fetching sp previews from cache"),
        }
    }
}

/// Generates a map page for a given map_id
/// OPTIONAL PARAMETER cat_id
///   Example endpoint  -> /map/sp/47802               Will assume default category ID
///                     -> /map/sp/47802?cat_id=40     Will use cat_id of 40
#[get("/map/sp/{map_id}")]
pub async fn get_singleplayer_maps(
    map_id: web::Path<String>,
    cat_id: web::Query<Opti32>,
    config: web::Data<Config>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = SpMap::get_sp_map_page(
        pool.get_ref(),
        map_id.clone(),
        config.proof.results,
        cat_id
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&map_id.into_inner()]),
    )
    .await;
    match res {
        Ok(sp_map) => {
            let mut ranked_vec = Vec::with_capacity(config.proof.results as usize);
            for (i, entry) in sp_map.into_iter().enumerate() {
                if i > 200 {
                    ranked_vec.push(SpRanked {
                        map_data: entry,
                        rank: i as i32 + 1,
                        points: score(i as i32 + 1),
                    });
                } else {
                    ranked_vec.push(SpRanked {
                        map_data: entry,
                        rank: i as i32 + 1,
                        points: 0.0,
                    });
                }
            }
            HttpResponse::Ok().json(ranked_vec)
        }
        _ => HttpResponse::NotFound().body("Error fetching SP Map Page"),
    }
}
/// Gives the profile number and score for all banned times on a given SP map
#[get("/maps/sp/banned/{map_id}")]
async fn get_banned_scores_sp(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = SpBanned::get_sp_banned(pool.get_ref(), map_id.to_string()).await;
    match res {
        Ok(banned_entries) => HttpResponse::Ok().json(banned_entries),
        _ => HttpResponse::NotFound().body("Error fetching SP Banned Player info."),
    }
}

/// Gives the profile number and score for all banned times on a given SP map
#[get("/sp/banned/{map_id}")]
async fn post_banned_scores_sp(
    map_id: web::Path<String>,
    params: web::Query<ScoreParams>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = Changelog::check_banned_scores(
        pool.get_ref(),
        map_id.clone(),
        params.score,
        params.profile_number.clone(),
        params
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&map_id.into_inner()]),
    )
    .await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching SP Banned Player info."),
    }
}

/// Returns a players PB history on an SP map
#[get("/map/sp/{map_id}/{profile_number}")]
async fn get_sp_pbs(info: web::Path<(String, String)>, pool: web::Data<PgPool>) -> impl Responder {
    let map_id = info.0.clone();
    let profile_number = info.1.clone();
    let user_data: UsersPage;
    // Get information for the player (user_name and avatar).
    let res = Users::get_user_data(pool.get_ref(), profile_number.clone()).await;
    // TODO: Handle the case where the is no user in the db
    match res {
        Ok(Some(res)) => user_data = res,
        Ok(None) => {
            return HttpResponse::Ok().json(SpPbHistory {
                user_name: None,
                avatar: None,
                pb_history: None,
            })
        }
        _ => return HttpResponse::NotFound().body("Error fetching User Data on given user."),
    }
    // Get Changelog data for all previous times.
    let res =
        Changelog::get_sp_pb_history(pool.get_ref(), profile_number.clone(), map_id.clone()).await;
    match res {
        Ok(changelog_data) => HttpResponse::Ok().json(SpPbHistory {
            user_name: Some(user_data.user_name),
            avatar: Some(user_data.avatar),
            pb_history: Some(changelog_data),
        }),
        Err(e) => {
            eprintln!("Could not find SP PB History -> {}", e);
            HttpResponse::Ok().json(SpPbHistory {
                user_name: None,
                avatar: None,
                pb_history: None,
            })
        }
    }
}

/// Receives a new score to add to the DB.
#[allow(unused_variables)]
#[post("/sp/post_score")]
async fn post_score_sp(
    params: web::Json<ChangelogInsert>,
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    // TODO: Handle demo uploads.
    // TODO: Working with sequence re-sync. Need to implement role-back.

    let res = Changelog::insert_changelog(pool.get_ref(), params.0).await;
    match res {
        Ok(id) => {
            // Invalide our sp_previews cache with the new score.
            let state_data = &mut cache.current_state.lock().await;
            let is_cached = state_data.get_mut("sp_previews").unwrap();
            *is_cached = false;
            HttpResponse::Ok().json(id)
        }
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::NotFound().body("Error adding new score to database.")
        }
    }
}

/// Receives new data to update an existing score.
#[put("/maps/sp/update")]
async fn put_score_sp(params: web::Json<Changelog>, pool: web::Data<PgPool>) -> impl Responder {
    // TODO: Handle demo uploads.
    let res = Changelog::update_changelog(pool.get_ref(), params.0).await;
    match res {
        Ok(changelog_entry) => HttpResponse::Ok().json(changelog_entry),
        _ => HttpResponse::NotFound().body("Error updating score."),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreLookup {
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
}
/// Wrapper for check_for_valid_score for the public facing API.
#[post("/newscore_details")]
pub async fn get_newscore_details(
    pool: web::Data<PgPool>,
    data: web::Json<ScoreLookup>,
    cache: web::Data<CacheState>,
    config: web::Data<Config>,
) -> impl Responder {
    let res = check_for_valid_score(
        pool.get_ref(),
        data.profile_number.clone(),
        data.score,
        data.map_id.clone(),
        config.proof.results,
        cache.into_inner().default_cat_ids[&data.map_id],
    )
    .await;
    match res {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(e) => {
            eprintln!("Error finding newscore details -> {:#?}", e);
            HttpResponse::NotFound().body("Error finding user")
        }
    }
}
