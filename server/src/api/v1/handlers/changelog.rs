use crate::models::models::{
    Changelog, ChangelogInsert, ChangelogPage, ChangelogQueryParams, SubmissionChangelog,
};
use crate::tools::cache::CacheState;
use crate::tools::config::Config;
use crate::tools::helpers::check_for_valid_score;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for changelog entiries. Utilizes `ChangelogQueryParrams` as an optional addition to the query
///
/// **Optional Parameters**: [ChangelogQueryParams]
/// ## Parameters:
///    - `limit`           
///         - The # of max returned results.
///    - `nick_name`       
///         - Filters for results from all profile_numbers were steam/board name matches `(%TEXT%)`.
///    - `profile_number`  
///         - Returns scores only from a specific profile (steam) id.
///    - `chamber`         
///         - Filters for only a specfic map by id.
///    - `sp`              
///         - Boolean for determines if sp maps should be returned
///    - `coop`            
///         - Boolean that determines if coop maps should be returned
///    - `wr_gain`         
///         - Boolean that, if true, will only return scores that were originally World Records
///    - `has_demo`        
///         - Boolean that will filter for only scores with demos
///    - `yt`              
///         - Boolean that will filter for onlny scores with youtube links
///    - `first`           
///         - Will only return scores with an ID higher than the given amount
///    - `last`            
///         - Will only return scores with an ID lower than the given amount
/// ## Example endpoints:
///  - **Default**           
///     - `/api/v1/changelog`
///  - **With parameters**   
///     - `/api/v1/changelog?limit=200&nick_name=Zypeh&chamber=47759&sp=true&coop=false&wr_gain=true&has_demo=true&yt=true`
///  - **A refresh call**    
///     - `/api/v1/changelog?limit=200&first=157804`
///  - **A scroll call**     
///     - `/api/v1/changelog?limit-200&last=157604`
///
/// Makes a call to the underlying [ChangelogPage::get_changelog_page]
#[get("/changelog")]
async fn changelog(
    pool: web::Data<PgPool>,
    query_params: web::Query<ChangelogQueryParams>,
) -> impl Responder {
    let res = ChangelogPage::get_changelog_page(pool.get_ref(), query_params.into_inner()).await;
    match res {
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}

/// **POST** endpoint for submitting a new changelog entry.
/// Accepts field values for a new changelog insert
/// ## Expects the following fields:
///
/// **Required Parameters**: timestamp, profile_number, score, map_id
///
/// **Optional Parameters**: youtube_id, note, cat_id
///
/// ## Parameters (expects valid JSON Object):
///
/// - `timestamp`    
///     - `String`: `%Y-%m-%d %H:%M:%S` (use `%20` to denote a space)
/// - `profile_number`
///     - `String`: Steam ID Number
/// - `score`         
///     - `i32`: Current board time format         
/// - `map_id`       
///     - `String`: Steam ID for the map
/// - `youtube_id`
///     - `String`: Youtube URL Extension.
/// - `note`          
///     - `String`: Note for the run
/// - `category_id`   
///     - `i32`: ID for the category being played  
///
/// ## Example endpoints:       
/// - `/api/v1/changelog`
///
/// ## Example JSON String
/// ```json
/// {
///     "timestamp" : "2020-08-18%2024:60:60",
///     "profile_number" : "76561198040982247",
///     "score" : 1763,
///     "map_id" : "47763",
///     "youtube_id" : null,
///     "note" : null,
///     "category_id" : 19,
/// }
/// ```
///
#[post("/changelog")]
async fn changelog_add(
    pool: web::Data<PgPool>,
    cache: web::Data<CacheState>,
    config: web::Data<Config>,
    params: web::Json<SubmissionChangelog>,
) -> impl Responder {
    let cache = cache.into_inner();
    let mut cl_insert =
        ChangelogInsert::new_from_submission(params.into_inner(), cache.default_cat_ids.clone())
            .await;
    let map_id = cl_insert.map_id.clone();
    match check_for_valid_score(
        pool.get_ref(),
        cl_insert.profile_number.clone(),
        cl_insert.score,
        cl_insert.map_id.clone(),
        config.proof.results,
        cl_insert.category_id,
    )
    .await
    {
        Ok(details) => {
            if !details.banned {
                cl_insert.previous_id = details.previous_id;
                cl_insert.post_rank = details.post_rank;
                cl_insert.pre_rank = details.pre_rank;
                cl_insert.score_delta = details.score_delta;
            } else {
                eprintln!("USER IS BANNED, DO NOT ADD A TIME FOR THEM");
                return HttpResponse::NotFound().body("User is banned");
            }
        }
        Err(e) => {
            eprintln!("Error finding newscore details -> {:#?}", e);
            return HttpResponse::NotFound().body("User not found, or better time exists.");
        }
    };
    match Changelog::insert_changelog(pool.get_ref(), cl_insert).await {
        Ok(id) => {
            // TODO: Add an endpoint to upload a coop time.
            cache
                .reload_rank(pool.get_ref(), &map_id, config.get_ref(), true)
                .await;
            HttpResponse::Ok().json(id)
        }
        Err(e) => {
            eprintln!("Error with adding changelog entry to database -> {}", e);
            HttpResponse::InternalServerError().body("Could not add user to databse")
        }
    }
}
