use actix_web::{get, body::Body, http::header, web, HttpResponse, Error};

/* Loads in a JSON file, converts and passes this JSON as the response.*/
#[get("/maps/sp/{mapid}")]
async fn levels_json(mapid: web::Path<u64>) -> Result<HttpResponse, Error> {
    let file  = format!("./api/maps/sp/{}.json", mapid.into_inner());
    Ok(HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(std::fs::read_to_string(file)?)))
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(levels_json)
    );
}