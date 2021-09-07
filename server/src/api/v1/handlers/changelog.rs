use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::tools::datamodels::ChangelogPage;
use crate::tools::datamodels::ChangelogQueryParams;

// TODO: We can consider making the limit higher in the future, or calculating it dynamically based off of scores in the last X amount of time.
/// GET method for most recent 200 changelog entries.
#[get("/changelog")]
async fn get_changelog(pool: web::Data<PgPool>) -> impl Responder{
    let limit: i32 = 200;
    let res = ChangelogPage::get_cl_page(pool.get_ref(), limit).await;
    match res{
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}

/// POST method for changelog that allows the user to submit a JSON body to filter for specific parameters. See the ChangelogQueryParams struct info on accepted query parameters.
#[post("/changelog")]
async fn post_changelog_filtered(params: web::Json<ChangelogQueryParams>, pool: web::Data<PgPool>) -> impl Responder{
    let res = ChangelogPage::get_cl_page_filtered(pool.get_ref(), params.0).await;
    match res{
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}