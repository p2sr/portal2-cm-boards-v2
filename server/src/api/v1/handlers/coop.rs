use crate::models::changelog::{Changelog, ScoreParams};
use crate::models::chapters::OptIDs;
use crate::models::coop::*;
use crate::tools::cache::{read_from_file, write_to_file, CacheState};
use crate::tools::{config::Config, helpers::filter_coop_entries};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

// TODO: Should use default cat_id
/// **GET** Returns top 7 information for each map, used to generate the previews page for Coop.
///
/// Inital load tends to be relatively slow, but the information cached, and
/// remains in chache until a new singleplayer score is submitted
///
/// ## Example Endpoints:
/// - **Default**
///     - `/api/v1/coop`
///
/// Makes a call to the underlying [CoopPreviews::get_coop_previews]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "map_id": "47741",
///         "scores": [
///             {
///                 "profile_number1": "76561198048179892",
///                 "profile_number2": "76561198095730281",
///                 "score": 1805,
///                 "youtube_id1": null,
///                 "youtube_id2": "z7vEUIkvsqI?start=0",
///                 "category_id": 62,
///                 "user_name1": "Betsruner",
///                 "user_name2": "Rex"
///             },...]},...}
/// ```
#[get("/coop")]
async fn coop(pool: web::Data<PgPool>, cache: web::Data<CacheState>) -> impl Responder {
    let state_data = &mut cache.current_state.lock().await;
    let is_cached = state_data.get_mut("coop_previews").unwrap();
    if !*is_cached {
        match CoopPreviews::get_coop_previews(pool.get_ref()).await {
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
        match read_from_file::<Vec<CoopPreviews>>("coop_previews").await {
            Ok(previews) => HttpResponse::Ok().json(previews),
            _ => HttpResponse::NotFound().body("Error fetching coop previews from cache"),
        }
    }
}

/// **GET** method that returns all coop scores for a maps page on a specific category.
///
/// Filtering of duplicate entries is handled.
///
/// ## Parameters:
/// - `cat_id`           
///     - **Optional** - `i32` : The ID of the category you want a Cooperative Ranked Page for.
/// - `cat_id`
///     - **Optional** - `i32` : The ID of the game, defaults to the base game (id = 1).
///
/// Example Endpoints:
/// - **Default**
///     - `/api/v1/map/coop/47741` - Will assume default category ID
/// - **Specific IDs**
///     - `/api/v1/map/coop/47741?cat_id=61&game_id=1`
///
/// Makes a call to the underlying [CoopMap::get_coop_map_page]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "map_data": {
///             "timestamp": "2021-02-21T02:12:42",
///             "profile_number1": "76561198048179892",
///             "profile_number2": "76561198095730281",
///             "score": 750,
///             "p1_is_host": null,
///             "demo_id1": 18944,
///             "demo_id2": 18945,
///             "youtube_id1": null,
///             "youtube_id2": "QPWf5CZ7mmk?start=0",
///             "submission1": false,
///             "submission2": false,
///             "note1": "Holy shit no reportal wtf",
///             "note2": null,
///             "category_id": 61,
///             "user_name1": "Betsruner",
///             "user_name2": "Rex",
///             "avatar1": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/2c/2c1253fe42835727f16e4625674a6079b963d16e_full.jpg",
///             "avatar2": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/d1/d11da394a941150d2e9ac9b8e0f9cf029e1d3b09_full.jpg"
///         },
///         "rank": 1,
///         "points": 200.0
///     },...]
/// ```
#[get("/map/coop/{map_id}")]
async fn coop_map(
    map_id: web::Path<String>,
    ids: web::Query<OptIDs>,
    config: web::Data<Config>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let map_id = map_id.into_inner();
    match CoopMap::get_coop_map_page(
        pool.get_ref(),
        &map_id,
        config.proof.results,
        ids.cat_id
            .unwrap_or_else(|| cache.into_inner().default_cat_ids[&map_id]),
        ids.game_id.unwrap_or(1),
    )
    .await
    {
        Ok(coop_entries) => {
            let coop_entries_filtered =
                filter_coop_entries(coop_entries, config.proof.results as usize).await;
            HttpResponse::Ok().json(coop_entries_filtered)
        }
        _ => HttpResponse::NotFound().body("Error fetching Coop Map Page"),
    }
}

/// **GET** method to return all banned scores on a map for a specific category.
///
/// ## Parameters
/// - `cat_id`
///     - **Optional** - `i32` : The ID of the category you want a Single Player Ranked Page for.
///
/// ## Example Endpoints:
/// - **Default**
///     - `/api/v1/coop/map_banned/47741`
/// - **Specific Category ID**
///     - `/api/v1/coop/map_banned/47741?cat_id=61`
///
/// Makes a call to the underlying [CoopBanned::get_coop_banned]
///
/// ## Example JSON output
/// ```json
/// [
///     {
///         "profile_number1": "76561198039912258",
///         "profile_number2": "76561198295368421",
///         "score": 986
///     },...]
/// ```
#[get("/coop/map_banned/{map_id}")]
async fn coop_banned_all(
    map_id: web::Path<String>,
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
    params: web::Query<OptIDs>,
) -> impl Responder {
    match CoopBanned::get_coop_banned(
        pool.get_ref(),
        map_id.clone(),
        params
            .cat_id
            .unwrap_or_else(|| cache.into_inner().default_cat_ids[&map_id.into_inner()]),
    )
    .await
    {
        Ok(banned_entries) => HttpResponse::Ok().json(banned_entries),
        _ => HttpResponse::NotFound().body("Error fetching Coop banned information"),
    }
}

// TODO: Handle differently for coop?
/// **GET** method to return a bool if a specific score is banned or not.
///
/// Currently this uses the same logic for SP times.
///
/// ## Parameters
/// - `map_id`
///     - **Required** - `String` : **Not a query string, part of the endpoint**
/// - `profile_number`
///     - **Required** - `String` : The profile_number of the user.
/// - `score`
///     - **Required** - `i32` : The score (time) associated with the run.
/// - `cat_id`
///     - **Optional** - `i32` : A specific category ID, if left blank will use the default.
/// - `game_id`
///     - **Optional** - `i32` : The ID for the game, will default to the basegame (id = 1)
///
/// Example Endpoints:
/// - **Default**
///     - `/api/v1/coop/time_banned/47825?profile_number=76561198823602829&score=1890`
/// - **Specific IDs**
///     - `/api/v1/coop/time_banned/47825?profile_number=76561198823602829&score=1890&cat_id=62&game_id=1`
///
/// ## Example JSON output
/// ```json
/// true
/// ```
#[get("/coop/time_banned/{map_id}")]
async fn coop_banned(
    map_id: web::Path<String>,
    params: web::Query<ScoreParams>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match Changelog::check_banned_scores(
        pool.get_ref(),
        map_id.clone(),
        params.score,
        params.profile_number.clone(),
        params
            .cat_id
            .unwrap_or_else(|| cache.into_inner().default_cat_ids[&map_id.into_inner()]),
        params.game_id.unwrap_or(1),
    )
    .await
    {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        Err(_) => HttpResponse::NotFound().body("Error checking ban information."),
    }
}
// pub p_id1: String,
// pub p_id2: Option<String>,
// pub p1_is_host: Option<bool>,
// pub cl_id1: i64,
// pub cl_id2: Option<i64>,
/// **POST** method that accepts a new coop score.
///
/// Makes the assumption that there are two existing changelog entries that will be used to create a new coop score.
///
/// ## Parameters:
/// - `p_id1`
///     - **Required** - `String` : The required profile_number for one of the users in a coop time.
/// - `p_id2`
///     - **Optional** - `String` : This is optional for backwards compatability, but for new times this should **not** be optional.
/// - `p1_is_host`
///     - **Optional** - `bool` : If `p_id1` was the host for the run.
/// - `cl_id1`
///     - **Required** - `i64` : The `changelog_id` for player 1's run.
/// - `cl_id2`
///     - **Optional** - `i64` : Same as `p_id2`, this should only be optional for backwards compatability, required for new scores.
///
/// ## Example Endpoints
/// - `/api/v1/coop/post_score`
///
/// Makes a call to the underlying [CoopBundled::insert_coop_bundled]
///
/// ## Example JSON string
/// ```json
/// {
///     "p_id1" : "76561197997838862",
///     "p_id2" : "76561198181126266",
///     "p1_is_host" : true,
///     "cl_id1" : 157752,
///     "cl_id2" : 157753
/// }
/// ```
#[post("/coop/post_score")]
async fn coop_add(
    params: web::Json<CoopBundledInsert>,
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    match CoopBundled::insert_coop_bundled(pool.get_ref(), params.0).await {
        Ok(id) => {
            // Invalidate cache if this new score impacts the top 7 preview times.
            let state_data = &mut cache.current_state.lock().await;
            let is_cached = state_data.get_mut("coop_previews").unwrap();
            *is_cached = false;
            HttpResponse::Ok().json(id)
        }
        _ => HttpResponse::NotFound().body("Error adding new score to database."),
    }
}

#[get("/coop/temp/{map_id}")]
async fn coop_temp(pool: web::Data<PgPool>, map_id: web::Path<String>) -> impl Responder {
    match CoopBundled::get_temp_coop_changelog(pool.get_ref(), &map_id).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error finding temp score -> {}", e);
            HttpResponse::NotFound().body("Cannot find temp score on given map.")
        }
    }
}
