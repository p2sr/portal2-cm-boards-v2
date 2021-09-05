use actix_web::{get, post, web, HttpResponse, Error};

use crate::tools::datamodels::ChangelogPage;
use crate::tools::datamodels::ChangelogQueryParams;

// NOTE: We can consider making the limit higher in the future, or calculating it dynamically based off of scores in the last X amount of time.
/// GET method for most recent 200 changelog entries.
#[get("/changelog")]
async fn get_changelog(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    let limit: i32 = 200;
    let changelog_entries = web::block(move || ChangelogPage::show(&conn, limit))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if let Some(changelog_entries) = changelog_entries{
        Ok(HttpResponse::Ok().json(changelog_entries))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}

/// POST method for changelog that allows the user to submit a JSON body to filter for specific parameters. See the ChangelogQueryParams struct info on accepted query parameters.
#[post("/changelog")]
async fn post_changelog_filtered(params: web::Json<ChangelogQueryParams>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("Could not get a DB connection from pool.");
    println!("Requested: {:#?}", params);
    let changelog_entries = web::block(move || ChangelogPage::show_filtered(&conn, params.nickname.clone(), params.profilenumber.clone(), params.chamber.clone(), params.sp, params.coop, params.wrgain, params.hasdemo, params.yt, params.limit))
    .await
    .map_err(|e|{
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if let Some(changelog_entries) = changelog_entries{
        Ok(HttpResponse::Ok().json(changelog_entries))
    } else {
        let res = HttpResponse::NotFound()
            .body("No changelog entries found.");
        Ok(res)
    }
}