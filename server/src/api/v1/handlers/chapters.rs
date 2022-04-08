use crate::models::models::{ChapterQueryParams, Chapters};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for map_ids by chapter.
///
/// **Required Parameters**: chapter_id.
///
/// Example Endpoints:
/// - **Default** - Using the chapter_id of `1`
///     - `/api/v1/chapter/1/maps`
#[get("/chapter/{chapter_id}/maps")]
async fn maps_from_chapter(chapter_id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let res = Chapters::get_map_ids(pool.get_ref(), chapter_id.into_inner()).await;
    match res {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}

/// **GET** method to return all chapters that match a specific name
///
/// ## Parameters
/// - **Required** Name search string (space separated by `%20`)
///
/// ## Example Endpoints:
/// - **With Search String** - Uses the search string
///     - `/api/v1/chapter/1`
#[get("/chapter/{chapter_id}")]
async fn chapter(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match Chapters::get_chapter_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}

/// **GET** method to return all chapters that match the provided [ChapterQueryParams].
///
///
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
