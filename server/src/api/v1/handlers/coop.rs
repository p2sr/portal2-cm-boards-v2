use crate::models::models::{
    Changelog, CoopBanned, CoopBundled, CoopBundledInsert, CoopMap, CoopPreviews, OptCatID,
    ScoreParams,
};
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

/// **GET** method that returns all coop scores for a maps page on a specific category.
///
/// Filtering of duplicate entries is handled.
///
/// ## Parameters:
/// - `cat_id`           
///     - **Optional** - `i32` : The ID of the category you want a Cooperative Ranked Page for.
///
/// Example Endpoints:
/// - **Default**
///     - `/api/v1/map/coop/47741` - Will assume default category ID
/// - **Specific Category**
///     - `/api/v1/map/coop/47741?cat_id=61`
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
    cat_id: web::Query<OptCatID>,
    config: web::Data<Config>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let map_id = map_id.into_inner();
    match CoopMap::get_coop_map_page(
        pool.get_ref(),
        &map_id,
        config.proof.results,
        cat_id
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&map_id]),
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
    params: web::Query<OptCatID>,
) -> impl Responder {
    let res = CoopBanned::get_coop_banned(
        pool.get_ref(),
        map_id.clone(),
        params
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&map_id.into_inner()]),
    )
    .await;
    match res {
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
///
/// Example Endpoints:
/// - **Default**
///     - `/api/v1/coop/time_banned/47825?profile_number=76561198823602829&score=1890`
/// - **Specific Category ID**
///     - `/api/v1/coop/time_banned/47825?profile_number=76561198823602829&score=1890&cat_id=62`
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
        Err(_) => HttpResponse::NotFound().body("Error checking ban information."),
    }
}

#[post("/coop/post_score")]
async fn coop_add(
    params: web::Json<CoopBundledInsert>,
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    let res = CoopBundled::insert_coop_bundled(pool.get_ref(), params.0).await;
    match res {
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
