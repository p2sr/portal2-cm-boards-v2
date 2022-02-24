use crate::controllers::models::ChangelogPage;
use crate::controllers::models::ChangelogQueryParams;

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

// pub limit: Option<u64>,
// pub nick_name: Option<String>,
// pub profile_number: Option<String>,
// pub chamber: Option<String>,
// pub sp: bool,
// pub coop: bool,
// pub wr_gain: Option<bool>,
// pub has_demo: Option<bool>,
// pub yt: Option<bool>,

// TODO: We can consider making the limit higher in the future, or calculating it dynamically based off of scores in the last X amount of time.
/// GET method for most recent 200 changelog entries.
#[get("/changelog")]
async fn get_changelog(
    pool: web::Data<PgPool>,
    query_params: web::Query<ChangelogQueryParams>,
) -> impl Responder {
    // let limit: i32 = 200;
    println!("{:#?}", query_params);
    let res = ChangelogPage::get_cl_page_filtered(pool.get_ref(), query_params.into_inner()).await;
    match res {
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}

// /// POST method for changelog that allows the user to submit a JSON body to filter for specific parameters. See the ChangelogQueryParams struct info on accepted query parameters.
// #[post("/changelog")]
// async fn post_changelog_filtered(
//     params: web::Json<ChangelogQueryParams>,
//     pool: web::Data<PgPool>,
// ) -> impl Responder {
//     let res = ChangelogPage::get_cl_page_filtered(pool.get_ref(), params.0).await;
//     match res {
//         Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
//         Err(e) => {
//             eprintln!("Error with filtering changelog page -> {}", e);
//             HttpResponse::NotFound().body("No changelog entries found.")
//         }
//     }
// }
