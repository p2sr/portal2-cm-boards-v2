//#![deny(missing_docs)]
#[macro_use]
extern crate serde_derive;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::{Error, Result};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::PgPool;

/// Module for the API versions containing handlers for API endpoints.
mod api;
/// Module for database interactions and models.
mod controllers;
/// Module for models
mod models;
/// Module for testing
mod tests;
/// Helpter functions used for the boards
mod tools;

/// Driver code to start and mount all compontents to the webserver we create.
#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Use config.rs to extract a configuration struct from .env (See documentation about changing .env.example)
    let config = crate::tools::config::Config::from_env().unwrap();
    println!("{:#?}", config);
    // Database pool, uses manager to build new database pool, saved in web::Data.
    // Reference Code: https://github.com/actix/examples/blob/master/database_interactions/diesel/src/main.rs
    let pool = PgPool::connect(&config.database_url).await?;

    // Initializes Logger with "default" format:  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
    // Remote-IP, Time, First line of request, Response status, Size of response in bytes, Referer, User-Agent, Time to serve
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host = config.server.host.clone();
    let port = config.server.port;
    println!(
        "Server starting at http://{}:{}/",
        config.server.host, config.server.port
    );
    // Get a map of map_ids to default category IDs.
    let default_cat_ids = crate::tools::helpers::get_default_cat_ids(&pool).await;
    // Construct the cache.
    let init_data = crate::tools::cache::CacheState::new(default_cat_ids);
    // Start our web server, mount and set up routes, data, wrapping, middleware and loggers
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(init_data.clone()))
            .configure(api::v1::handlers::init::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}
