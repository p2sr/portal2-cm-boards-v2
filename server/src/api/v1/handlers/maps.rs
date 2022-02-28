use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::controllers::models::Maps;

#[get("/category/default_category/{map}")]
async fn get_default_cat(params: web::Path<u64>, pool: web::Data<PgPool>) -> impl Responder {
    let res = Maps::get_default_cat(pool.get_ref(), params.to_string()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(id),
        _ => HttpResponse::NotFound().body("Error finding deafult cat_id"),
    }
}
