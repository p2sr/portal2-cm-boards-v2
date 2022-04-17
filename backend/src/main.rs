// #![allow(clippy::all)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
#![allow(mutable_borrow_reservation_conflict)]

#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use chrono::prelude::*;
use dotenv::dotenv;
use env_logger::Env;
use points::*;
use rayon::prelude::*;
use stages::fetching::*;
// use time::PreciseTime;

mod models;
mod points;
mod stages;
mod tests;

const OFFICIAL_SP: [i32; 51] = [
    47458, 47455, 47452, 47106, 47735, 47736, 47738, 47742, 47744, 47465, 47746, 47748, 47751,
    47752, 47755, 47756, 47759, 47760, 47763, 47764, 47766, 47768, 47770, 47773, 47774, 47776,
    47779, 47780, 47783, 47784, 47787, 47468, 47469, 47472, 47791, 47793, 47795, 47798, 47800,
    47802, 47804, 47806, 47808, 47811, 47813, 47815, 47817, 47819, 47821, 47824, 47456,
];

const OFFICIAL_COOP: [i32; 48] = [
    47741, 47825, 47828, 47829, 45467, 46362, 47831, 47833, 47835, 47837, 47840, 47841, 47844,
    47845, 47848, 47849, 47854, 47856, 47858, 47861, 52642, 52660, 52662, 52663, 52665, 52667,
    52671, 52687, 52689, 52691, 52777, 52694, 52711, 52714, 52715, 52717, 52735, 52738, 52740,
    49341, 49343, 49345, 49347, 49349, 49351, 52757, 52759, 48287,
];

const LIMIT_MULT_SP: i32 = 2;
const LIMIT_MULT_COOP: i32 = 3;

/// Driver code to start and mount all compontents to the webserver we create.
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // Initializes Logger with "default" format:  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
    // Remote-IP, Time, First line of request, Response status, Size of response in bytes, Referer, User-Agent, Time to serve
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host = dotenv::var("HOST").expect("Cannot find HOST in ./.env");
    let port = dotenv::var("PORT").expect("Cannot find PORT in ./.env");
    let limit: i32 = dotenv::var("LIMIT")
        .expect("Cannot find LIMIT in ./.env")
        .parse::<i32>()
        .unwrap();
    // Get a map of map_ids to default category IDs.
    println!("Server starting at http://{}:{}/", host, port);
    // Start our web server, mount and set up routes, data, wrapping, middleware and loggers
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(limit))
            .configure(init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}

#[get("/test")]
pub async fn test() -> HttpResponse {
    HttpResponse::Ok().body("hello world")
}

#[get("/recalculate_points/{id}")]
pub async fn rcp(id: web::Path<i32>) -> HttpResponse {
    // Assume that a negative `id` means that the user wants to recalculate all points.
    web::block(move || {
        let id = id.into_inner();
        if id < 0 {
            calc_points(None);
        } else {
            // Get a list of maps altered? In this case it would probably just be the ID.
            calc_points(Some(vec![id]));
        }
    })
    .await
    .unwrap();
    HttpResponse::Ok().body("Success.")
}

// TODO: It might be much faster to just grab all the data from the web server at the start.
// Given how long it takes to go through and query for each of the like, thousands of potential times.
#[get("/fetch_all")]
pub async fn fetch_all(limit: web::Data<i32>) -> HttpResponse {
    web::block(move || {
        let limit = limit.into_inner();
        let utc = Utc::now().naive_utc();
        let _res_sp: Vec<_> = OFFICIAL_SP
            .into_par_iter()
            .map(|map_id| fetch_entries(map_id, 0, *limit * LIMIT_MULT_SP, utc, false))
            .collect();
        let _res_cp: Vec<_> = OFFICIAL_COOP
            .into_par_iter()
            .map(|map_id| fetch_entries(map_id, 0, *limit * LIMIT_MULT_COOP, utc, true))
            .collect();
    })
    .await
    .unwrap();
    HttpResponse::Ok().body("Success.")
}

// TODO: It looks like currently, there is a conflict between the async thread pool, and the threadpool for rayon, as this is now VERY slow.
// TODO: Seems to currenty upload without caring about new user? Post new user information to `/user`.
#[get("/fetch_sp/{map_id}")]
pub async fn fetch_sp(map_id: web::Path<i32>, limit: web::Data<i32>) -> HttpResponse {
    web::block(move || {
        let limit = limit.into_inner();
        let utc = Utc::now().naive_utc();
        fetch_entries(map_id.into_inner(), 0, *limit * LIMIT_MULT_SP, utc, false);
    })
    .await
    .unwrap();
    HttpResponse::Ok().body("Success.")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/backend/v1")
            .service(test)
            .service(rcp)
            .service(fetch_sp),
    );
}

// fn fetch_all(limit: i32) {
//     let official_sp = [
//         47458, 47455, 47452, 47106, 47735, 47736, 47738, 47742, 47744, 47465, 47746, 47748, 47751,
//         47752, 47755, 47756, 47759, 47760, 47763, 47764, 47766, 47768, 47770, 47773, 47774, 47776,
//         47779, 47780, 47783, 47784, 47787, 47468, 47469, 47472, 47791, 47793, 47795, 47798, 47800,
//         47802, 47804, 47806, 47808, 47811, 47813, 47815, 47817, 47819, 47821, 47824, 47456,
//     ];

//     let official_coop = [
//         47741, 47825, 47828, 47829, 45467, 46362, 47831, 47833, 47835, 47837, 47840, 47841, 47844,
//         47845, 47848, 47849, 47854, 47856, 47858, 47861, 52642, 52660, 52662, 52663, 52665, 52667,
//         52671, 52687, 52689, 52691, 52777, 52694, 52711, 52714, 52715, 52717, 52735, 52738, 52740,
//         49341, 49343, 49345, 49347, 49349, 49351, 52757, 52759, 48287,
//     ];

//     let utc = Utc::now().naive_utc();
//     let _res_sp: Vec<_> = official_sp
//         .into_par_iter()
//         .map(|map_id| fetch_entries(map_id, 0, limit * LIMIT_MULT_SP, utc, false))
//         .collect();
//     let _res_cp: Vec<_> = official_coop
//         .into_par_iter()
//         .map(|map_id| fetch_entries(map_id, 0, limit * LIMIT_MULT_COOP, utc, true))
//         .collect();

//     // What do we do with the leaderboards...
// }

// fn fetch_sp(map_id: String, limit: i32) {
//     let utc = Utc::now().naive_utc();
//     let _res_sp = fetch_entries(
//         map_id.parse().expect("Error parsing map_id"),
//         0,
//         limit * LIMIT_MULT_SP,
//         utc,
//         false,
//     );
//     // Recalculate the points on the given map. Force reset cache on webserver.
//     // Setup an endpoint on the webserver to invalidate cache for a specific map.
// }
// fn fetch_cp(map_id: String, limit: i32) {
//     let utc = Utc::now().naive_utc();
//     let _res_coop = fetch_entries(
//         map_id.parse().expect("Error parsing map_id"),
//         0,
//         limit * LIMIT_MULT_COOP,
//         utc,
//         true,
//     );
// }
