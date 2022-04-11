use crate::models::models::{GameID, Maps};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method to return all map information for a given game.
///
/// ## Parameters:
/// - `game_id`
///     - **Optional** - `i32` : ID for the game that the map/chapter belongs to.
///                              If left empty, defaults to base-game (`id` = 1)
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/maps`
///  - **With game_id**
///     - `/api/v1/maps?game_id=1`
///
/// Makes a call to the underlying [Maps::get_maps]
///
/// ## Example JSON output
///
/// ``` json
/// [
///     {
///         "id": 51,
///         "steam_id": "47458",
///         "lp_id": "47459",
///         "name": "Portal Gun",
///         "chapter_id": 7,
///         "default_cat_id": 1,
///         "is_public": true
///     },...]
/// ```
#[get("/maps")]
async fn maps(pool: web::Data<PgPool>, query: web::Query<GameID>) -> impl Responder {
    match Maps::get_maps(pool.get_ref(), query.into_inner().game_id.unwrap_or(1)).await {
        Ok(m) => HttpResponse::Ok().json(m),
        Err(e) => {
            eprintln!("Error getting all maps -> {}", e);
            HttpResponse::NotFound().body("Error retrieving all map information")
        }
    }
}

/// **GET** method to return the default category ID for a given map
///
/// ## Example endpoints:
///  - **Default**
///     - `/api/v1/default_cateogry/47458`
///
/// Makes a call to the underlying [Maps::get_maps]
///
/// ## Example JSON ouput
///
/// ```json
/// 1
/// ```
#[get("/default_category/{map}")]
async fn default_category(params: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    match Maps::get_default_cat(pool.get_ref(), params.to_string()).await {
        Ok(id) => HttpResponse::Ok().json(id),
        _ => HttpResponse::NotFound().body("Error finding deafult cat_id"),
    }
}
