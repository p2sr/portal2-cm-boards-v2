use crate::models::models::{Users, UsersDisplay};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

/// GET the user information for a given profile_number.
#[get("/users/{profile_number}")]
async fn get_user(pool: web::Data<PgPool>, profile_number: web::Path<String>) -> impl Responder {
    let res = Users::get_user(pool.get_ref(), profile_number.into_inner()).await;
    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User does not exist."),
        _ => HttpResponse::NotFound().body("Error fetching Users"),
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
        _ => HttpResponse::NotFound().body("Error fetching banned users"),
    }
}

/// POST method for adding a new user to the db.
#[post("/users")]
async fn post_new_user(pool: web::Data<PgPool>, new_user: web::Json<Users>) -> impl Responder {
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
async fn get_donators(pool: web::Data<PgPool>) -> impl Responder {
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
async fn get_wall_of_shame(pool: web::Data<PgPool>) -> impl Responder {
    let res = Users::get_banned_display(pool.get_ref()).await;
    match res {
        Ok(Some(users)) => HttpResponse::Ok().json(users),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

//TODO: #[get("/profile/{profile_number}")]
// async fn get_profile(pool: web::Data<PgPool>) -> impl Responder {
//     let res = ;
// }
