use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::tools::datamodels::{Users, UserParams};

/// GET method for the steamIDs of all banned users on the board.
#[get("/bannedusers")]
async fn get_banned_users(pool: web::Data<PgPool>) -> impl Responder{
    let res = Users::get_banned(pool.get_ref()).await;
    match res{
        Ok(profile_numbers) => HttpResponse::Ok().json(profile_numbers),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}

/// POST method to check the ban status of a given player.
#[post("/bannedusers")]
async fn check_ban_status(params: web::Json<UserParams>, pool: web::Data<PgPool>) -> impl Responder{
    let res = Users::check_banned(pool.get_ref(), params.profile_number.clone()).await;
    match res{
        Ok(banned_bool) => HttpResponse::Ok().json(banned_bool),
        _ => HttpResponse::NotFound().body("Error fetching previews"),
    }
}