use crate::models::models::{PointsProfileWrapper, ProfilePage, Users};
use crate::tools::cache::CacheState;
use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::Result;
use sqlx::PgPool;
use std::collections::HashMap;

/// GET the user information for a given profile_number.
#[get("/users/{profile_number}")]
async fn users(pool: web::Data<PgPool>, profile_number: web::Path<String>) -> impl Responder {
    let res = Users::get_user(pool.get_ref(), profile_number.into_inner()).await;
    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User does not exist."),
        _ => HttpResponse::NotFound().body("Error fetching Users"),
    }
}

// TODO: Change banned user endpoints
/// GET method for the steamIDs of all banned users on the board.
#[get("/banned_users_all")]
async fn banned_users_all(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned(pool.get_ref()).await;
    match res {
        Ok(profile_numbers) => HttpResponse::Ok().json(profile_numbers),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

// TODO: Update this to be a GET Mathod
/// POST method to check the ban status of a given player.
#[post("/banned_user")]
async fn banned_user(profile_number: web::Json<String>, pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::check_banned(pool.get_ref(), profile_number.into_inner().clone()).await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching banned users"),
    }
}

/// POST method for adding a new user to the db.
#[post("/users")]
async fn users_add(pool: web::Data<PgPool>, new_user: web::Json<Users>) -> impl Responder {
    let res = Users::insert_new_users(pool.get_ref(), new_user.0.clone()).await;
    match res {
        Ok(true) => HttpResponse::Ok().json(new_user.0),
        Ok(false) => HttpResponse::InternalServerError().body("Could not add user to database"),
        Err(e) => {
            eprintln!(
                "Adding user {:#?} to DB failed with error -> {}",
                new_user.0, e
            );
            HttpResponse::InternalServerError().body("Could not add user to database.")
        }
    }
}
// TODO: Fix the naming
#[get("/donators")]
async fn donators(pool: web::Data<PgPool>) -> impl Responder {
    match Users::get_donators(pool.get_ref()).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error getting donation stats -> {}", e);
            HttpResponse::NotFound().body("Could not find donation stats.")
        }
        _ => HttpResponse::NotFound().body("Could not find donation stats."),
    }
}

/// GET method for the UsersDisplay of all banned users on the board.
#[get("/wall_of_shame")]
async fn wall_of_shame(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned_display(pool.get_ref()).await;
    match res {
        Ok(Some(u)) => HttpResponse::Ok().json(u),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

#[get("/profile/{profile_number}")]
async fn profile(
    pool: web::Data<PgPool>,
    profile_number: web::Path<String>,
    cache: web::Data<CacheState>,
) -> impl Responder {
    let profile_number = profile_number.into_inner();
    match Users::get_profile(pool.get_ref(), &profile_number).await {
        Ok(Some(data)) => match profile_from_cache(cache, &profile_number).await {
            Ok((points, ranks)) => {
                let profile_page = ProfilePage {
                    points,
                    ranks,
                    data,
                };
                HttpResponse::Ok().json(profile_page)
            }
            Err(e) => {
                eprintln!("Error creating profile page -> {:?}", e);
                HttpResponse::NotFound().body("Could find profile page for user.")
            }
        },
        Err(e) => {
            eprintln!("Error creating profile page -> {:?}", e);
            HttpResponse::NotFound().body("Could find profile page for user.")
        }
        _ => HttpResponse::NotFound().body("Could find profile page for user."),
    }
}

pub async fn profile_from_cache(
    cache: web::Data<CacheState>,
    profile_number: &String,
) -> Result<(Vec<PointsProfileWrapper>, HashMap<String, i32>)> {
    let mut points: Vec<PointsProfileWrapper> = Vec::new();
    let points_hm = cache.points.lock().await;
    for i in 1..16 {
        if let Some(points_cache) = points_hm.get(&*format!("points{}", i)) {
            if let Some(x) = points_cache.get(profile_number) {
                points.push(PointsProfileWrapper {
                    id: i,
                    points: x.clone(),
                });
            }
        }
    }
    if let Some(points_cache) = points_hm.get("points_sp") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -1,
                points: x.clone(),
            });
        }
    }
    if let Some(points_cache) = points_hm.get("points_coop") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -2,
                points: x.clone(),
            });
        }
    }
    if let Some(points_cache) = points_hm.get("points_overall") {
        if let Some(x) = points_cache.get(profile_number) {
            points.push(PointsProfileWrapper {
                id: -3,
                points: x.clone(),
            });
        }
    }
    let r = &*cache.ranks.lock().await;
    let ranks = r.current_ranks.get(profile_number).unwrap().clone();
    Ok((points, ranks))
}
