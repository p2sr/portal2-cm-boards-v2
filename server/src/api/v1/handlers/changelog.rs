use crate::models::changelog::*;
use crate::models::demos::DemoOptions;
use crate::tools::cache::CacheState;
use crate::tools::config::Config;
use crate::tools::helpers::get_valid_changelog_insert;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

/// **GET** method for changelog entiries. Utilizes [ChangelogQueryParams] as an optional addition to the query
///
/// ## Parameters:
///    - `limit`           
///         - **Optional** - `i32` : The # of max returned results.
///    - `nick_name`       
///         - **Optional** - `String` : Filters for results from all profile_numbers were steam/board name matches `(%TEXT%)`.
///    - `profile_number`  
///         - **Optional** - `String` : Returns scores only from a specific profile (steam) id.
///    - `chamber`         
///         - **Optional** - `String` : Filters for only a specfic map by id.
///    - `sp`              
///         - **Optional** - `bool` : Determines if sp maps should be returned
///    - `coop`            
///         - **Optional** - `bool` : Ddetermines if coop maps should be returned
///    - `wr_gain`         
///         - **Optional** - `bool` : If true, will only return scores that were originally World Records
///    - `has_demo`        
///         - **Optional** - `bool` : Filters for only scores with demos
///    - `yt`              
///         - **Optional** - `bool` : Filters for onlny scores with youtube links
///    - `first`           
///         - **Optional** - `i64` : Will only return scores with an ID higher than the given amount
///    - `last`            
///         - **Optional** - `i64` : Will only return scores with an ID lower than the given amount
///
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
///
/// ## Example JSON output
/// ```json
/// [
///     {
///         "id": 99237,
///         "timestamp": "2019-07-19T17:33:39",
///         "profile_number": "76561198039230536",
///         "score": 2001,
///         "map_id": "47759",
///         "demo_id": 5932,
///         "banned": false,
///         "youtube_id": "-c0gaEXuKZA?start=0",
///         "previous_id": 99125,
///         "coop_id": null,
///         "post_rank": 1,
///         "pre_rank": 3,
///         "submission": false,
///         "note": null,
///         "category_id": 17,
///         "score_delta": -7,
///         "verified": true,
///         "admin_note": null,
///         "map_name": "Laser Relays",
///         "user_name": "Zypeh",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f9/f934276c99d0f970fdcb2d4e1229dde02d778d99_full.jpg"
///     },...]
/// ```
#[get("/changelog")]
async fn changelog(
    pool: web::Data<PgPool>,
    query_params: web::Query<ChangelogQueryParams>,
) -> impl Responder {
    match ChangelogPage::get_changelog_page(pool.get_ref(), query_params.into_inner()).await {
        Ok(changelog_entries) => HttpResponse::Ok().json(changelog_entries),
        _ => HttpResponse::NotFound().body("No changelog entries found."),
    }
}

/// **POST** method for submitting a new changelog entry.
///
/// Accepts field values for a new [SubmissionChangelog]
///
/// ## Parameters (expects valid JSON Object):
/// - `timestamp`    
///     - **Required** - `String` : `%Y-%m-%d %H:%M:%S` (use `%20` to denote a space)
/// - `profile_number`
///     - **Required** - `String` : Steam ID Number
/// - `score`         
///     - **Required** - `i32` : Current board time format         
/// - `map_id`       
///     - **Required** - `String` : Steam ID for the map
/// - `youtube_id`
///     - **Optional** - `String` : Youtube URL Extension.
/// - `note`          
///     - **Optional** - `String` : Note for the run
/// - `category_id`   
///     - **Optional** - `i32` : ID for the category being submitted, will use default for the map if not supplied,
/// - `game_id`
///     - **Optional** - `i32` : ID for the game, will default to base game (id = 1).
///
/// ## Example endpoints:       
/// - `/api/v1/changelog`
///
/// ## Example JSON Input String
/// ```json
/// {
///    "timestamp" : "2020-08-18%2024:60:60",
///     "profile_number" : "76561198040982247",
///     "score" : 1763,
///     "map_id" : "47763",
///     "youtube_id" : null,
///     "note" : null,
///     "category_id" : 67,
///     "game_id" : 1
/// } 
/// ```
#[post("/changelog")]
pub async fn changelog_new(pool: web::Data<PgPool>, cl: web::Json<SubmissionChangelog>, cache: web::Data<CacheState>, config: web::Data<Config>) -> impl Responder {
    let cache = cache.into_inner();
    let cl_i = match get_valid_changelog_insert(pool.get_ref(), &config.into_inner(), &cache, cl.into_inner()).await {
        Ok(insert) => insert,
        Err(e) => {
            eprintln!("Error validating changelog -> {e}");
            return HttpResponse::UnprocessableEntity().body("Could not validate changelog entry.")
        }
    };
    match Changelog::insert_changelog(pool.get_ref(), cl_i).await {
        Ok(id) => {
            // Invalidate both SP and coop caches
            let state_data = &mut cache.current_state.lock().await;
            let is_cached = state_data.get_mut("coop_previews").unwrap();
            *is_cached = false;
            let is_cached = state_data.get_mut("sp_previews").unwrap();
            *is_cached = false;
            HttpResponse::Ok().json(id)
        },
        Err(e) => {
            eprintln!("Error inserting changelog entry into database -> {e}");
            HttpResponse::UnprocessableEntity().body("Could not insert score into database.")
        }
    }
}

/// **GET** method for getting a hashmap of all default categories.
///
/// ## Example Endpoint:       
/// - `/api/v1/default_categories_all`
///
/// ## Example JSON Input String
/// ```json
/// {
///     "47813": 45,
///     "52738": 98,
///     "48287": 108,...}
/// ```
#[get("/default_categories_all")]
pub async fn default_categories_all(pool: web::Data<PgPool>) -> impl Responder {
    web::Json(crate::tools::helpers::get_default_cat_ids(pool.get_ref()).await)
}

/// **PUT** method for updating the demo_id on a changelog entry.
///
/// Accepts field values for a new [DemoOptions]. 
/// 
/// **Note** - [DemoOptions] was designed to be used to handle a choice between a demo/changelog ID. 
/// It is reused here for the sake of reducing the number of redundant structs. 
/// Both are optional in Rust, but required in this specific funciton.
///
/// ## Parameters (expects valid JSON Object):
/// - `demo_id`    
///     - **Required** - `i64` : The ID of the existing demo.
/// - `cl_id
///     - **Required** - `i64` : The ID of the existing changelog entry.
///
/// ## Example endpoints:       
/// - `/api/v1/changelog/demo`
///
/// ## Example JSON Input String
/// ```json
/// {
///     "cl_id" : 15625,
///     "demo_id" : 1251
/// }
/// ```
#[put("/changelog/demo")]
pub async fn changelog_demo_update(
    pool: web::Data<PgPool>,
    ids: web::Json<DemoOptions>,
    cache: web::Data<CacheState>
) -> impl Responder {
    let ids = ids.into_inner();
    match Changelog::update_demo_id_in_changelog(
        pool.get_ref(),
        ids.cl_id.unwrap(),
        ids.demo_id.unwrap(),
    )
    .await
    {
        Ok(b) => {
            // Invalidate both SP and coop caches
            let state_data = &mut cache.current_state.lock().await;
            let is_cached = state_data.get_mut("coop_previews").unwrap();
            *is_cached = false;
            let is_cached = state_data.get_mut("sp_previews").unwrap();
            *is_cached = false;
            HttpResponse::Ok().json(b)
        },
        Err(e) => {
            eprintln!("Error updating demo_id in changelog entry -> {e}");
            HttpResponse::InternalServerError().body("Error updating changelog entry.")
        }
    }
}
