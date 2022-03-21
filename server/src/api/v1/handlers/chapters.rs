use crate::models::models::Chapters;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for map_ids by chapter.
///
/// **Required Parameters**: chapter_id.
///
/// Example Endpoints:
/// - **Default** - Using the chapter_id of `1`
///     - `/api/v1/maps_from_chapter/1`
#[get("/maps_from_chapter/{chapter_id}")]
async fn get_map_ids_by_chapter(
    chapter_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = Chapters::get_map_ids(pool.get_ref(), chapter_id.into_inner()).await;
    match res {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}
/// **GET** method to return all chapters that match a specific name
///
/// **Required Parameters**: Name search string (space separated by `%20`)
///
/// Example Endpoints:
/// - **Default** - Uses the search string
///     - `/api/v1/chapters/The%20Part`
#[get("/chapters/{name}")]
async fn get_chapter_by_name(name: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", name);
    let res = Chapters::get_chapter_by_name(pool.get_ref(), name.into_inner()).await;
    match res {
        Ok(Some(s)) => HttpResponse::Ok().json(s),
        _ => HttpResponse::NotFound().body("No maps found for given chapter_id."),
    }
}
