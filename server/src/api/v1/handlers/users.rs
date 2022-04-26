use crate::models::models::{PointsProfileWrapper, ProfilePage, Users};
use crate::tools::cache::CacheState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use anyhow::Result;
use sqlx::PgPool;
use std::collections::HashMap;

/// **GET** method for user information for a specific `profile_number`.
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/user/76561198040982247`
///
/// Makes a call to the underlying [Users::get_user]
///
/// ## Example JSON output
///
/// ```json
/// {
///     "profile_number": "76561198040982247",
///     "board_name": "Daniel",
///     "steam_name": "BigDaniel",
///     "banned": false,
///     "registered": 0,
///     "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg",
///     "twitch": "bigdaniel",
///     "youtube": "/channel/UCtwF46_PUGCefgRfrcIXOZA",
///     "title": null,
///     "admin": 1,
///     "donation_amount": null,
///     "discord_id": null
/// }
/// ```
#[get("/user/{profile_number}")]
async fn user(pool: web::Data<PgPool>, profile_number: web::Path<String>) -> impl Responder {
    let res = Users::get_user(pool.get_ref(), profile_number.into_inner()).await;
    match res {
        Ok(Some(u)) => HttpResponse::Ok().json(u),
        Ok(None) => HttpResponse::NotFound().body("User does not exist."),
        _ => HttpResponse::NotFound().body("Error fetching Users"),
    }
}

/// **GET** method to get all `profile_number`s of all banned users on the board.
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/banned_users_all`
///
/// Makes a call to the underlying [Users::get_banned]
///
/// ## Example JSON output
///
/// ```json
/// [
///     "76561197960339145",
///     "76561197960403893",
///     "76561197961322276",
///     "76561197961957434",
///     "76561197962813887",
///     "76561197963045874",
///     "76561197965946552",...]
/// ```
#[get("/banned_users_all")]
async fn banned_users_all(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned(pool.get_ref()).await;
    match res {
        Ok(profile_numbers) => HttpResponse::Ok().json(profile_numbers),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// **GET** method to return a bool based on if a user is banned or not.
///
/// ## Parameters
/// - `profile_number`
///     - **Required** - `String` - ID of the user you want to check.
///
/// ## Example endpoints:
///  - **With Parameters**
///     - `/api/v1/banned_user?profile_number=76561197960339145`
///
/// Makes a call to the underlying [Users::check_banned]
///
/// ## Example JSON output
///
/// ``` json
/// true
/// ```
#[get("/banned_user")]
async fn banned_user(
    profile_number: web::Query<String>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = Users::check_banned(pool.get_ref(), profile_number.into_inner().clone()).await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching banned users"),
    }
}

/// **POST** method to upload a new user to the boards.
///
/// Accepts field values for a new [Users]
///
/// ## Parameters (expects valid JSON Object):
///
/// - `profile_number`    
///     - **Required** - `String` : Steam ID for the user.
/// - `steam_name`    
///     - **Required** - `String` : User's name on Steam (not technically required, but treated as required).
/// - `banned`    
///     - **Required** - `bool` : If the user is banned on the boards.
/// - `registered`    
///     - **Required** - `i32` : If the user is registered (should typically be false).
/// - `admin`    
///     - **Required** - `i32` : The user's admin level (0 is default)
/// - `board_name`    
///     - **Optional** - `String` : Name specific to the portal 2 boards, allowed to be changed by user.
/// - `avatar`
///     - **Optional** - `String` : The profile image used for the user. Typically pulled from steam.
/// - `twitch`
///     - **Optional** - `String` : Twitch username (not full URL).
/// - `youtube`
///     - **Optional** - `String` : YouTube account. Format -> `/user/{user_id}`
/// - `title``
///     - **Optional** - `String` : Unique title to be displayed on the boards.
/// - `donation_amount`
///     - **Optional** - `String` : Total amount of money donated to the boards.
/// - `discord_id`
///     - **Optional** - `String` : Discord tag for the user's discord account.
///
/// ## Example endpoints:       
/// - `/api/v1/user`
///
/// ## Example JSON String
/// ```json
/// {
///     "profile_number": "76561198040982247",
///     "board_name": "Daniel",
///     "steam_name": "BigDaniel",
///     "banned": false,
///     "registered": 0,
///     "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg",
///     "twitch": "bigdaniel",
///     "youtube": "/channel/UCtwF46_PUGCefgRfrcIXOZA",
///     "title": "Developer",
///     "admin": 1,
///     "donation_amount": null,
///     "discord_id": BigDaniel#9055
/// }
/// ```
#[post("/user")]
async fn user_add(pool: web::Data<PgPool>, new_user: web::Json<Users>) -> impl Responder {
    let res = Users::insert_new_users(pool.get_ref(), new_user.0.clone()).await;
    match res {
        Ok(true) => HttpResponse::Ok().json(new_user.0),
        Ok(false) => HttpResponse::InternalServerError().body("Could not add user to database"),
        Err(e) => {
            eprintln!(
                "Adding user {:#?} to DB failed with error -> {}",
                new_user.0, e
            );
            HttpResponse::InternalServerError().body("Could not add user to database.")
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AvatarInsert {
    pub avatar: String,
}

#[put("/user/avatar/{profile_number}")]
async fn avatar_update(
    pool: web::Data<PgPool>,
    data: web::Json<AvatarInsert>,
    profile_number: web::Path<String>,
) -> impl Responder {
    let avatar = data.into_inner().avatar;
    let profile_number = profile_number.into_inner();
    match Users::update_avatar(pool.get_ref(), &profile_number, &avatar).await {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => {
            eprintln!("Error updating avatar for user {} -> {}", profile_number, e);
            HttpResponse::NotModified().body("Failure at updating avatar.")
        }
    }
}

/// **GET** method to return all user information for donators on the boards.
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/donators`
///
/// Makes a call to the underlying [Users::get_donators]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "profile_number": "76561198029488151",
///         "board_name": null,
///         "steam_name": "BreweryJake",
///         "banned": false,
///         "registered": 0,
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/e0/e0b43f4bb265cdfc5e2486c67215b1082054e1a9_full.jpg",
///         "twitch": "breweryjake",
///         "youtube": "/user/HCMonstrLP",
///         "title": "Donator",
///         "admin": 0,
///         "donation_amount": "101",
///         "discord_id": null
///     },...]
/// ```
#[get("/donators")]
async fn donators(pool: web::Data<PgPool>) -> impl Responder {
    match Users::get_donators(pool.get_ref()).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error getting donation stats -> {}", e);
            HttpResponse::NotFound().body("Could not find donation stats.")
        }
        _ => HttpResponse::NotFound().body("Could not find donation stats."),
    }
}

/// **GET** method to return [UsersDisplay](crate::models::models::UsersDisplay) for all banned users on the boards.
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/wall_of_shame`
///
/// Makes a call to the underlying [Users::get_banned_display]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "profile_number": "76561197960339145",
///         "user_name": "louis vuitton",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f6/f6963c618e35e95ca84c631fcf7d6bf8dec72485_full.jpg"
///     },...]
/// ```
#[get("/wall_of_shame")]
async fn wall_of_shame(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned_display(pool.get_ref()).await;
    match res {
        Ok(Some(u)) => HttpResponse::Ok().json(u),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// **GET** method for most of a given user's profile information.
///
/// The user's changelog history can be grabbed from specifying filters to the
/// [Changelog](crate::api::v1::handlers::changelog::changelog) endpoint. This endpoint does
/// include information on the current ranks for all maps on the default category IDs per-map.
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/profile/76561198040982247`
///
/// Makes a call to the underlying [Users::get_profile], then utilizes the [cache](crate::tools::cache::CacheState)
/// to get rank information per-map.
///
/// ## Example JSON output
///
/// ```json
/// {
///     "points": [
///         {
///             "id": 1,
///             "points": {
///                 "points": 961.515,
///                 "score": 8653,
///                 "num_scores": 6,
///                 "total_rank_sum": 133,
///                 "worst": [
///                     36,
///                     "45467"
///                 ],
///                 "best": [
///                     12,
///                     "47828"
///                 ],
///                 "user_name": "Daniel",
///                 "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg"
///             }
///         },
///         ...],
///        "data": {
///            "oldest": {
///                "map": "52759",
///                "map_name": "Gel Maze",
///                "score": 2768,
///                "timestamp": "2019-04-18T20:51:22"
///            },
///            "newest": {
///                "map": "47755",
///                "map_name": "Bridge the Gap",
///                "score": 1628,
///                "timestamp": "2021-08-22T18:36:59"
///            }
///        },
///        "ranks": {
///            "47798": 24,
///            "47806": 19,
///         ...
///        }
///    }
/// ```
#[get("/profile/{profile_number}")]
async fn profile(
    pool: web::Data<PgPool>,
    profile_number: web::Path<String>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    // TODO : Scores on drop down are queried individually by the frontend
    let profile_number = profile_number.into_inner();
    match Users::get_profile(pool.get_ref(), &profile_number).await {
        Ok(Some(data)) => match profile_from_cache(cache, &profile_number).await {
            Ok((points, ranks)) => {
                let profile_page = ProfilePage {
                    points,
                    ranks,
                    data,
                };
                HttpResponse::Ok().json(profile_page)
            }
            Err(e) => {
                eprintln!("Error creating profile page -> {:?}", e);
                HttpResponse::NotFound().body("Could find profile page for user.")
            }
        },
        Err(e) => {
            eprintln!("Error creating profile page -> {:?}", e);
            HttpResponse::NotFound().body("Could find profile page for user.")
        }
        _ => HttpResponse::NotFound().body("Could find profile page for user."),
    }
}

/// Pulls & clones the data from the ranks cache to be used for the profile endpoint.
pub async fn profile_from_cache(
    cache: web::Data<CacheState>,
    profile_number: &String,
) -> Result<(Vec<PointsProfileWrapper>, HashMap<String, i32>)> {
    let mut points: Vec<PointsProfileWrapper> = Vec::new();
    let points_hm = cache.points.lock().await;
    for i in 1..16 {
        if let Some(points_cache) = points_hm.get(&*format!("points{}", i)) {
            if let Some(x) = points_cache.get(profile_number) {
                points.push(PointsProfileWrapper {
                    id: i,
                    points: x.clone(),
                });
            }
        }
    }
    if let Some(points_cache) = points_hm.get("points_sp") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -1,
                points: x.clone(),
            });
        }
    }
    if let Some(points_cache) = points_hm.get("points_coop") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -2,
                points: x.clone(),
            });
        }
    }
    if let Some(points_cache) = points_hm.get("points_overall") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -3,
                points: x.clone(),
            });
        }
    }
    let r = &*cache.ranks.lock().await;
    let ranks = r.current_ranks.get(profile_number).unwrap().clone();
    Ok((points, ranks))
}
