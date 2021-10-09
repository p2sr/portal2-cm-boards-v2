use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::tools::datamodels::Chapters;

// TODO: We can consider making the limit higher in the future, or calculating it dynamically based off of scores in the last X amount of time.
/// GET method for most recent 200 changelog entries.
#[get("/chapters/{chatper_id}")]
async fn get_map_ids_by_chapter(chapter_id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder{
    let res = Chapters::get_map_ids(pool.get_ref(), chapter_id.into_inner()).await;
    let res = match res{
        Ok(s) => s,
        _ => None,
    };
    match res{
        Some(map_ids) => HttpResponse::Ok().json(map_ids),
        None => HttpResponse::NotFound().body("No changelog entries found."),
    }
}
