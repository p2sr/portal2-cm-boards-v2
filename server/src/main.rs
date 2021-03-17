#[allow(unused_imports)]
use actix_web::{get, body::Body, http::header, web, App, HttpRequest, HttpServer,HttpResponse, Responder, Error};
/*
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}*/


// Load in file as json
// serde read in file as json
// Specify you're returning that json
#[get("/maps/sp/{mapid}")]
async fn levels_json(mapid: web::Path<u64>) -> Result<HttpResponse, Error> {
    let file  = format!("./api/maps/sp/{}.json", mapid.into_inner());
    // serde readfromfile(filepath);
    Ok(HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(std::fs::read_to_string(file)?)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(levels_json)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    //"api/maps/sp/{mapid}", web::get().to(levels_json)
    // GET /api/chambers/sp/{map}
    // GET /api/chambers/coop/{map}
    // POST /api/users/{id}
}

/*
    PUT REQUEST front-end team: {steamid: 1239842y5628347652, boardname: "Kevin"}
*/