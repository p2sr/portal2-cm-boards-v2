//! # Overview
//! The main web-server compontent of the boards. Written in Rust, this is the primary
//! REST API that the boards uses to interact with the database
//!
//! ## Navigation
//!
//! ### [API](api::v1::handlers)
//!
//! The [API](api::v1::handlers) module contains the different API versions, and the versions contain handles for the endpoints.
//!
//! The handlers are broken up into categories that follow closely with their purpose area on the boards. Each
//! category will contain different endpoints. Each endpoint should have usage documentation. **This is my recommendation on where to go
//! if you're looking to use the API's endpoints for your application.**
//!  
//! ### Controllers
//! The [controllers] source code contains the code for Database Interactions. The documentation here is virtually non-existant however, as
//! the code is written as methods implemented on models. So view individual modules in [models](models::models), or check
//! the API documention to see what underlying calls the API endpoints make.
//!
//! ### Models
//! The [models](models::models) that represent the data we extract from the database, and most modules used internally in the code. These have good information
//! on high-level purpose and any implementations on those models. This is where you would go to see what db interactions exist for a given
//! data model.
//!
//! ### Tests
//! The [testing](tests) module does not contain any useful documentation, and is used for testing development.
//!
//! ### Tools
//!
//! #### Helpers
//! [Miscellaneous functions](tools::helpers) used by different portions of the code
//!
//! #### Cache
//! [Caching implementations](tools::cache) specific to the board, well documented.
//!
//! #### Config
//! Extracts [configuration](tools::config) information from the local .env file to be used to customize boards. Includes networking information,
//! proof requirements for the boards, connection information for the database and external file servers etc.
//!
#![allow(rustdoc::private_intra_doc_links)]
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

#[cfg(test)]
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
    // Get a map of map_ids to default category IDs.
    let default_cat_ids = crate::tools::helpers::get_default_cat_ids(&pool).await;
    // Construct the cache.
    let init_data = crate::tools::cache::CacheState::new(&pool, &config, default_cat_ids).await;
    println!(
        "Server starting at http://{}:{}/",
        config.server.host, config.server.port
    );
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
