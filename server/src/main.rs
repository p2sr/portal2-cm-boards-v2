#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;


use std::error::Error;
use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::PgPool;
// use anyhow::{Result, Error};
// use env_logger::Env;
use dotenv::dotenv;

/// Configuration module that handles extracting information from the environment for setup.
mod config;
/// Module for the API versions containing handlers for API endpoints.
//mod api;
/// Module for tools like our models and some of the calculation functions we use for the boards.
//mod tools;

#[get("/")]
fn index() -> &'static str{
    "Hello world!"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let pool = PgPool::connect(&config.database_url).await;

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::build().mount("/", routes![index]).launch().await?;
}
// Driver code to start and mount all compontents to the webserver we create.
// #[actix_web::main]
// async fn main() -> Result<(), Error> {
//     dotenv().ok();
//     // Use config.rs to extract a configuration struct from .env (See documentation about changing .env.example)
//     let config = crate::config::Config::from_env().unwrap();
//     // Database pool, uses manager to build new database pool, saved in web::Data.
//     // Reference Code: https://github.com/actix/examples/blob/master/database_interactions/diesel/src/main.rs
    
//     let pool = PgPool::connect(&config.database_url).await?;

//     // Initializes Logger with "default" format:  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
//     // Remote-IP, Time, First line of request, Response status, Size of response in bytes, Referer, User-Agent, Time to serve
//     std::env::set_var("RUST_LOG", "actix_web=info");
//     env_logger::Builder::from_env(Env::default()
//         .default_filter_or("info"))
//         .init();

//     println!("Server starting at http://{}:{}/", config.server.host, config.server.port);
//     // Start our web server, mount and set up routes, data, wrapping, middleware and loggers
//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allowed_origin("http://localhost:3000")
//             .allowed_methods(vec!["GET"])
//             .max_age(3600);
//         App::new()
//             .wrap(cors)
//             .wrap(Logger::default())
//             .app_data(web::Data::new(pool.clone()))
//             .configure(api::v1::handlers::init::init)
//         })
//         .bind(format!("{}:{}", config.server.host, config.server.port))?
//         .run()
//         .await?;
//     Ok(())
// }
