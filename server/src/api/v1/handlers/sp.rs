use crate::models::models::{
    Changelog, HistoryParams, OptCatID, ScoreLookup, ScoreParams, SpBanned, SpMap, SpPbHistory,
    SpPreviews, SpRanked, Users, UsersPage,
};
use crate::tools::cache::{read_from_file, write_to_file, CacheState};
use crate::tools::helpers::check_for_valid_score;
use crate::tools::{config::Config, helpers::score};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// TODO: Invalidate cache when a time is banned/verified/when a player is banned.
/// **GET** method to handle the preview page showing all singleplayer maps.
///
/// Inital load tends to be relatively slow, but the information cached, and
/// remains in chache until a new singleplayer score is submitted
///
/// ## Example endpoints:
///  - **Default**           
///     - `/api/v1/sp`
///
/// Makes a call to the underlying [SpPreviews::get_sp_previews]
/// **or** uses a cached value.
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "map_id": "47458",
///         "scores": [
///             {
///                 "profile_number": "76561198795823814",
///                 "score": 2326,
///                 "youtube_id": "DPgJgmLmzCw?start=0",
///                 "category_id": 1,
///                 "user_name": "Royal",
///                 "map_id": "47458"
///             },...]}]
/// ```
#[get("/sp")]
async fn sp(pool: web::Data<PgPool>, cache: web::Data<CacheState>) -> impl Responder {
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

// TODO: Add game
/// **GET** method to generate a single player map page [SpRanked] for a given map_id
///
/// ## Parameters:
///    - `cat_id`           
///         - **Optional** - `i32` - The ID of the category you want a Single Player Ranked Page for.
///
/// ## Example endpoint
/// - **Default**
///     - `/api/v1/map/sp/47802` - Will assume default category ID
/// - **Specific Category**                   
///     - `/api/v1/map/sp/47802?cat_id=40`
///
/// Makes a call to the underlying [SpMap::get_sp_map_page].
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "map_data": {
///             "timestamp": "2021-04-28T06:51:16",
///             "profile_number": "76561198254956991",
///             "score": 1729,
///             "demo_id": 21885,
///             "youtube_id": "MtwWXAO2E5c?start=0",
///             "submission": false,
///             "note": "https://www.youtube.com/watch?v=orwgEEaJln0",
///             "category_id": 40,
///             "user_name": "Zyntex",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/9d/9d160bcde456f7bb452b1ed9d9e740cd73f89266_full.jpg"
///         },
///         "rank": 1,
///         "points": 200.0
///     },....]
/// ```
#[get("/map/sp/{map_id}")]
pub async fn sp_map(
    map_id: web::Path<String>,
    cat_id: web::Query<OptCatID>,
    config: web::Data<Config>,
    cache: web::Data<CacheState>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let map_id = map_id.into_inner();
    let res = SpMap::get_sp_map_page(
        pool.get_ref(),
        &map_id,
        config.proof.results,
        cat_id
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&map_id]),
    )
    .await;
    match res {
        Ok(sp_map) => {
            // Check cache
            let mut ranked_vec = Vec::with_capacity(config.proof.results as usize);
            for (i, entry) in sp_map.into_iter().enumerate() {
                // TODO: Fix tied ranks.
                ranked_vec.push(SpRanked {
                    map_data: entry,
                    rank: i as i32 + 1,
                    points: score(i as i32 + 1),
                })
            }
            HttpResponse::Ok().json(ranked_vec)
        }
        _ => HttpResponse::NotFound().body("Error fetching SP Map Page"),
    }
}
/// **GET** method to return the profile number and score for all banned times on a given singleplayer map.
///
/// ## Example Endpoins
/// - **Default**
///     - `/api/v1/sp/all_banned/47458`
///
/// Makes a call to the underlying [SpBanned::get_sp_banned]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "profile_number": "76561197961322276",
///         "score": -2147483648
///     },
///     {
///         "profile_number": "76561198096964328",
///         "score": -2147483648
///     }
/// ]
/// ```
#[get("/sp/all_banned/{map_id}")]
async fn sp_all_banned(map_id: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = SpBanned::get_sp_banned(pool.get_ref(), map_id.to_string()).await;
    match res {
        Ok(banned_entries) => HttpResponse::Ok().json(banned_entries),
        _ => HttpResponse::NotFound().body("Error fetching SP Banned Player info."),
    }
}
/// **GET** method to return true or false given a `map_id`, `profile_number` and `score`
///
/// ## Parameters:
///    - `map_id`
///         - Required: Part of the endpoint, **not** a part of the query string.
///    - `profile_number`           
///         - Required: `String`, ID for the player.
///    - `score`           
///         - Required: `i32`, Time for the run.
///
/// ## Example Endpoins
/// - **With Parameters**
///     - `/api/v1/sp/banned/47458?profile_number=76561198823602829&score=2445`
///
/// Makes a call to the underlying [SpBanned::get_sp_banned]
///
/// ## Example JSON output
///
/// ```json
/// true
/// ```
#[get("/sp/banned/{map_id}")]
async fn sp_banned(
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

/// **GET** method to return a history of scores on a current map, for a given player.
///
/// Query parameters represented as [HistoryParams]
///
/// ## Parameters:
///    - `profile_number`           
///         - Required: `String`, ID for the player.
///    - `map_id`           
///         - Required: `String`, ID for the map.
///    - `cat_id`           
///         - Optional: `String`, ID for the category.
///
/// ## Example Endpoints:
/// - **With Parametes**
///     - `/api/v1/sp/history?map_id=47458&profile_number=76561198795823814
/// - **With cat_id**
///     - `/api/v1/sp/history?map_id=47458&profile_number=76561198795823814&cat_id=1
///
/// Makes a call to the underlying [Users::get_user_data] & [Changelog::get_sp_pb_history]
///
/// # Example JSON output
///
/// - For a user that exists
///
/// ```json
/// {
///     "user_name": "Royal",
///     "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/d8/d84366b1be1f0439b0edc7fc8404fe2ea29a9c54_full.jpg",
///     "pb_history": [
///         {
///             "id": 152184,
///             "timestamp": "2021-07-06T09:11:04",
///             "profile_number": "76561198795823814",
///             "score": 2326,
///             "map_id": "47458",
///             "demo_id": 24527,
///             "banned": false,
///             "youtube_id": "DPgJgmLmzCw?start=0",
///             "previous_id": 141996,
///             "coop_id": null,
///             "post_rank": 1,
///             "pre_rank": 1,
///             "submission": true,
///             "note": "",
///             "category_id": 1,
///             "score_delta": -7,
///             "verified": true,
///             "admin_note": null
///         },..]}
/// ```
///
/// - For a user that does not exist.
///
/// ```json
/// {
///     "user_name": null,
///     "avatar": null,
///     "pb_history": null
/// }
/// ```
#[get("/sp/history")]
async fn sp_history(
    query: web::Query<HistoryParams>,
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    let query = query.into_inner();
    let user_data: UsersPage;
    // Get information for the player (user_name and avatar).
    let res = Users::get_user_data(pool.get_ref(), &query.profile_number).await;
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
    let res = Changelog::get_sp_pb_history(
        pool.get_ref(),
        &query.profile_number,
        &query.map_id,
        query
            .cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&query.map_id]),
    )
    .await;
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

// /// Receives a new score to add to the DB.
// #[allow(unused_variables)]
// #[post("/sp/post_score")]
// async fn post_score_sp(
//     params: web::Json<ChangelogInsert>,
//     pool: web::Data<PgPool>,
//     cache: web::Data<CacheState>,
// ) -> impl Responder {
//     // TODO: Handle demo uploads.
//     // TODO: Working with sequence re-sync. Need to implement role-back.

//     let res = Changelog::insert_changelog(pool.get_ref(), params.0).await;
//     match res {
//         Ok(id) => {
//             // Invalide our sp_previews cache with the new score.
//             let state_data = &mut cache.current_state.lock().await;
//             let is_cached = state_data.get_mut("sp_previews").unwrap();
//             *is_cached = false;
//             HttpResponse::Ok().json(id)
//         }
//         Err(e) => {
//             eprintln!("{}", e);
//             HttpResponse::NotFound().body("Error adding new score to database.")
//         }
//     }
// }

// TODO: Make this more ergonomic? Don't require all values.
// TODO: Authentication should impact what a user can update.
// TODO: Update to return all.
/// **PUT** Method to update data for an existing singleplayer score.
///
/// Expects a JSON object as input. Best practice is to pass the current JSON [Changelog] object, and alter the fields you want changed.
///
/// ## Parameters:
/// - `id`
///     - **Required** : `i64` : The ID of the changelog entry you want to update.
/// - `timestamp`    
///     - **Required** : `String` : `%Y-%m-%d %H:%M:%S` (use `%20` to denote a space)
/// - `profile_number`
///     - **Required** : `String` : Steam ID Number
/// - `score`         
///     - **Required** : `i32` : Current board time format         
/// - `map_id`       
///     - **Required** : `String` : Steam ID for the map
/// - `banned`
///     - **Required** : `bool` : If the score is banned.
/// - `submission`
///     - **Required** : `bool` : If the score is a submission.
/// - `category_id`   
///     - `i32` : ID for the category being played.
/// - `demo_id`
///     - **Optional** : `i64` : ID for the associated demo.
/// - `youtube_id`
///     - **Optional** : `String`: Youtube URL Extension.
/// - `previous_id`
///     - **Optional** : `i64` : Previous score ID for the user.
/// - `coop_id`
///     - **Optional** : `i64` : Coop ID for the score.
/// - `post_rank`
///     - **Optional** : `i32` : Rank when submitted.
/// - `pre_rank`
///     - **Optional** : `i32` : Previous Rank when the new score was submitted.
/// - `note`          
///     - **Optional** : `String` : User comment for the run.
/// - `score_delta`
///     - **Optional** : `i32` : Difference in score between the two entries.
/// - `verified`
///     - **Optional** : `bool` : If the run is verified.
/// - `admin_note`
///     - **Optional** : `String` : Note by admin.
///
/// Makes a call to the underlying [Changelog::update_changelog]
///
/// ## Example JSON output
///
/// ```json
/// true
/// ```
#[put("/sp/update")]
async fn sp_update(params: web::Json<Changelog>, pool: web::Data<PgPool>) -> impl Responder {
    // TODO: Handle demo uploads.
    let res = Changelog::update_changelog(pool.get_ref(), params.0).await;
    match res {
        Ok(changelog_entry) => HttpResponse::Ok().json(changelog_entry),
        _ => HttpResponse::NotFound().body("Error updating score."),
    }
}

/// **GET** method for validating an SP Score. Mainly used by our backend that pulls times from the Steam leaderboards.
///
/// Query parameters represented as [ScoreLookup]
///
/// ## Parameters:
///    - `profile_number`           
///         - **Required**: `String`, ID for the player.
///    - `score`           
///         - **Required**: `i32`, Time for the run.
///    - `map_id`           
///         - **Required**: `String`, ID for the map.
///    - `cat_id`           
///         - **Optional**: `i32`, ID for the cateogry. If left blank, will use the default for the map.
///
/// ## Example endpoints:
///  - **With Required**           
///     - `/api/v1/sp/validate?profile_number=76561198039230536&score=2346&map_id=47458`
///  - **With cat_id**   
///     - `/api/v1/sp/validate?profile_number=76561198039230536&score=2346&map_id=47458&?cat_id=1`
///
/// Makes a call to the underlying [check_for_valid_score]
///
/// ## Example JSON output:
///
/// ```json
/// {
///     "previous_id": 102347,
///     "post_rank": 500,
///     "pre_rank": 3,
///     "score_delta": 2,
///     "banned": false
/// }
/// ```
#[get("/sp/validate")]
pub async fn sp_validate(
    pool: web::Data<PgPool>,
    data: web::Query<ScoreLookup>,
    cache: web::Data<CacheState>,
    config: web::Data<Config>,
) -> impl Responder {
    let res = check_for_valid_score(
        pool.get_ref(),
        data.profile_number.clone(),
        data.score,
        data.map_id.clone(),
        config.proof.results,
        data.cat_id
            .unwrap_or(cache.into_inner().default_cat_ids[&data.map_id]),
    )
    .await;
    match res {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(e) => {
            eprintln!("Error finding newscore details -> {:#?}", e);
            HttpResponse::NotFound().json("Score is not valid.")
        }
    }
}
