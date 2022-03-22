use crate::models::models::Users;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

/// GET the user information for a given profile_number.
#[get("/users/{profile_number}")]
async fn get_user(pool: web::Data<PgPool>, profile_number: web::Path<String>) -> impl Responder {
    let res = Users::get_user(pool.get_ref(), profile_number.into_inner()).await;
    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User does not exist."),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// GET method for the steamIDs of all banned users on the board.
#[get("/banned_users")]
async fn get_banned_users(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned(pool.get_ref()).await;
    match res {
        Ok(profile_numbers) => HttpResponse::Ok().json(profile_numbers),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// POST method to check the ban status of a given player.
#[post("/banned_users")]
async fn check_ban_status(
    profile_number: web::Json<String>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let res = Users::check_banned(pool.get_ref(), profile_number.into_inner().clone()).await;
    match res {
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// POST method for adding a new user to the db.
#[post("/new_user")]
async fn post_new_user(pool: web::Data<PgPool>, new_user: web::Json<Users>) -> impl Responder {
    let res = Users::insert_new_users(pool.get_ref(), new_user.0.clone()).await;
    match res {
        Ok(true) => HttpResponse::Ok().json(new_user.0),
        Ok(false) => HttpResponse::InternalServerError().body("Could not add user to database"),
        Err(e) => {
            eprintln!(
                "Adding user {:?} to DB failed with error -> {}",
                new_user.0, e
            );
            HttpResponse::InternalServerError().body("Could not add user to database.")
        }
    }
}
