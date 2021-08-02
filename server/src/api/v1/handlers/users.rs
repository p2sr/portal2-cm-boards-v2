use actix_web::{get, post, web, HttpResponse, Error};

use crate::db::DbPool;
use crate::tools::datamodels::{Usersnew, UserParams};

/// GET method for the steamIDs of all banned users on the board.
#[get("/bannedusers")]
async fn get_banned_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    
    let banned_users = web::block(move || Usersnew::showbanned(&conn))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(banned_users))
}

/// POST method to check the ban status of a given player.
#[post("/bannedusers")]
async fn check_ban_status(params: web::Json<UserParams>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    println!("Requested ban status of user: {:?}", params);
    let user_status = web::block(move || Usersnew::check_banned(&conn, params.profilenumber.clone()))
    .await
    .map_err(|e|{
        eprintln!("{:#?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(user_status))
}