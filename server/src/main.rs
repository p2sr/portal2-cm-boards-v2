use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


// Load in file as json
// serde read in file as json
// Specify you're returning that json


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    // GET /api/chambers/sp/{map}
    // GET /api/chambers/coop/{map}
    // POST /api/users/{id}
}

/*
    PUT REQUEST front-end team: {steamid: 1239842y5628347652, boardname: "Kevin"}
*/