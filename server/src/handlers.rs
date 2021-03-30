use actix_web::{get, body::Body, http::header, web, HttpResponse, Error};

use crate::db::DbPool;
use crate::models::Changelog;

/* Loads in a JSON file, converts and passes this JSON as the response.*/
#[get("/maps/sp/{mapid}")]
async fn levels_json(mapid: web::Path<u64>) -> Result<HttpResponse, Error> {
    let file  = format!("./api/maps/sp/{}.json", mapid.into_inner());
    Ok(HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(std::fs::read_to_string(file)?)))
}

#[get("/test/sp/")]
async fn dbpool_test(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let cl = web::block(move || Changelog::all(&conn))
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(cl) = cl{
        Ok(HttpResponse::Ok().json(cl))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(levels_json)
            .service(dbpool_test)
    );
}