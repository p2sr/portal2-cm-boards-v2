use crate::models::models::{ChapterQueryParams, Chapters};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for map_ids by chapter.
///
/// ## Parameters:
/// - `chapter_id`
///     - **Required** - `i32` : Used in the actual endpoint. Follows `v1/chapter/{chapter_id}/maps`
///
/// Example Endpoints:
/// - **Default** - Using the chapter_id of `1`
///     - `/api/v1/chapter/1/maps`
///
/// ## Example JSON output
/// ```json
/// [
///     "47741",
///     "47825",
///     "47828",
///     "47829",
///     "45467",
///     "46362"
/// ]
/// ```
#[get("/chapter/{chapter_id}/maps")]
async fn maps_from_chapter(chapter_id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match Chapters::get_map_ids(pool.get_ref(), chapter_id.into_inner()).await {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}

/// **GET** method to return all chapters that match a specific name
///
/// ## Parameters
/// - `chapter_id`
///     - **Required** - `i64` : Used in the actual endpoint. Follows `v1/chapter/{chapter_id}`
///
/// ## Example Endpoints:
/// - **With Search String** - Uses the search string
///     - `/api/v1/chapter/1`
///
/// ## Example JSON output
/// ```json
/// {
///     "id": 1,
///     "chapter_name": "Team Building",
///     "is_multiplayer": true,
///     "game_id": 1
/// }
/// ```
#[get("/chapter/{chapter_id}")]
async fn chapter(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match Chapters::get_chapter_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}

/// **GET** method to return all chapters, can filter using parameters in [ChapterQueryParams].
///
/// ## Parameters:
/// - `chapter_name`
///     - **Optional** - `String` : Will match on any portion of the name.
/// - `is_multiplayer`
///     - **Optional** - `bool` : If the chapter is multiplayer or not. Default is both.
/// - `game_id`
///     - **Optional** - `i32` : ID of the game. Defaults to all.
///
/// ## Example Endpoints
/// - **Default**
///     - `/api/v1/chapters` - Will return all chapters
/// - **With Params**
///     - `/api/v1/chapters?chapter_name=Art%20T&is_multiplayer=true&game_id=1`
///
/// ## Example JSON output
/// ```json
/// [
///     {
///         "id": 6,
///         "chapter_name": "Art Therapy",
///         "is_multiplayer": true,
///         "game_id": 1
///     }
/// ]
/// ```
#[get("/chapters")]
async fn chapters_filtered(
    params: web::Query<ChapterQueryParams>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match Chapters::get_filtered_chapters(pool.get_ref(), params.into_inner()).await {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given parameters."),
    }
}
