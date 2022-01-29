use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::tools::calc::score;
use crate::tools::datamodels::{
    Changelog, ChangelogInsert, ScoreParams, SpBanned, SpMap, SpPbHistory, SpPreviews, SpRanked,
    Users, UsersPage
};

/// GET endpoint to handle the preview page showing all sp maps.
/// Returns: Json wrapped values -> {map_id, scores{ steam_id, profile_number, score, youtube_id, category_id, user_name } }
#[get("/sp")]
async fn get_singleplayer_preview(pool: web::Data<PgPool>) -> impl Responder {
    let res = SpPreviews::get_sp_previews(pool.get_ref()).await;
    match res {
        Ok(sp_previews) => HttpResponse::Ok().json(sp_previews),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// Calls models::SpMap to grab the entries for a particular map_id, returns a vector of the top 200 times, in a slimmed down fashion (only essential data)
/// Handles filtering out obsolete times (1 time per runner)
#[get("/maps/sp/{map_id}")]
async fn get_singleplayer_maps(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = SpMap::get_sp_map_page(pool.get_ref(), map_id.to_string()).await;
    match res {
        Ok(sp_map) => {
            let mut i = 1;
            let mut ranked_vec = Vec::with_capacity(200);
            for entry in sp_map {
                ranked_vec.push(SpRanked {
                    map_data: entry,
                    rank: i,
                    points: score(i),
                });
                i += 1;
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
#[post("/maps/sp/banned/{map_id}")]
async fn post_banned_scores_sp(
    map_id: web::Path<u64>,
    params: web::Json<ScoreParams>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = Changelog::check_banned_scores(
        pool.get_ref(),
        map_id.to_string(),
        params.score,
        params.profile_number.clone(),
    )
    .await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching SP Banned Player info."),
    }
}

/// Returns a players PB history on an SP map
#[get("/maps/sp/{map_id}/{profile_number}")]
async fn get_sp_pbs(info: web::Path<(String, String)>, pool: web::Data<PgPool>) -> impl Responder {
    let map_id = info.0.clone();
    let profile_number = info.1.clone();
    let user_data: UsersPage;
    // Get information for the player (user_name and avatar).
    let res = Users::get_user_data(pool.get_ref(), profile_number.clone()).await;
    match res {
        Ok(Some(res)) => user_data = res,
        Ok(None) => return HttpResponse::Ok().json(SpPbHistory {
            user_name: None,
            avatar: None,
            pb_history: None,
        }),
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
        _ => HttpResponse::NotFound().body("Error fetching Changelog data on given user."),
    }
}

/// Receives a new score to add to the DB.
#[post("/sp/post_score")]
async fn post_score_sp(
    params: web::Json<ChangelogInsert>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // TODO: Handle demo uploads.
    // TODO: Fix this not working
    // let res = Changelog::insert_changelog(pool.get_ref(), params.0).await;
    // match res {
    //     Ok(id) => HttpResponse::Ok().json(id),
    //     Err(e) => {
    //         eprintln!("{}",e);
    //         HttpResponse::NotFound().body("Error adding new score to database.")
    //     },
    // }
    // TEMP WORK AROUND FOR TESTING
    let id = 1;
    HttpResponse::Ok().json(id)
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