use crate::{
    models::{admin::*, changelog::ChangelogQueryParams, users::Users},
    tools::error::Result,
};
use actix_web::{get, web, Responder};
use sqlx::PgPool;

/// **GET** method for admin-relevant entries. Utilizes [ChangelogQueryParams] as an optional addition to the query
///
/// ## Parameters:
///    - `limit`           
///         - **Optional** - `i32` : The # of max returned results.
///    - `nick_name`       
///         - **Optional** - `String` : Searches for any matches to the given string. Not case sensitive.
///    - `profile_number`  
///         - **Optional** - `String` : Returns scores only from a specific profile (steam) id.
///    - `chamber`         
///         - **Optional** - `i32` : Filters for only a specific `map_id`.
///    - `sp`              
///         - **Optional** - `bool` : Determines if sp maps should be returned.
///    - `coop`            
///         - **Optional** - `bool` : Determines if coop maps should be returned.
///    - `wr_gain`         
///         - **Optional** - `bool` : If true, will only return scores that were originally World Records
///    - `has_demo`        
///         - **Optional** - `bool` : Filter for only scores with demos
///    - `yt`              
///         - **Optional** - `bool` : Filter for only scores with youtube links
///    - `first`           
///         - **Optional** - `i64` : Will only return scores with an ID higher than the given amount
///    - `last`            
///         - **Optional** - `i64` : Will only return scores with an ID lower than the given amount
/// ## Example endpoints:
///  - **Default**           
///     - `/api/v1/admin/changelog`
///  - **With parameters**   
///     - `/api/v1/admin/changelog?limit=200&nick_name=Zypeh&chamber=47759&sp=true&coop=false&wr_gain=true&has_demo=true&yt=true`
///  - **A refresh call**    
///     - `/api/v1/admin/changelog?limit=200&first=157804`
///  - **A scroll call**     
///     - `/api/v1/admin/changelog?limit-200&last=157604`
///
/// Makes a call to the underlying [Admin::get_admin_page]
///
/// ## Example JSON output
/// ```json
/// [
///     {
///         "id": 157795,
///         "timestamp": "2021-08-25T09:53:11",
///         "profile_number": "76561199114333959",
///         "score": 2273,
///         "map_id": "47472",
///         "demo_id": null,
///         "banned": true,
///         "youtube_id": null,
///         "previous_id": null,
///         "coop_id": null,
///         "post_rank": 1,
///         "pre_rank": null,
///         "submission": false,
///         "note": "Ban Reason - Used Give (Daniel)",
///         "category_id": 34,
///         "score_delta": null,
///         "verified": false,
///         "admin_note": null,
///         "map_name": "PotatOS",
///         "user_name": "HackerKnownAsRan",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/79/79d3fe5839617eb83a9661071ed021dd56ac8a5b_full.jpg"
///     },...]
/// ```
#[get("/admin/changelog")]
pub async fn admin_changelog(
    pool: web::Data<PgPool>,
    query_params: web::Query<ChangelogQueryParams>,
) -> Result<impl Responder> {
    Ok(web::Json(
        Admin::get_admin_page(pool.get_ref(), query_params.into_inner()).await?,
    ))
}

/// **GET** method for user statistics on total times, banned times and non-verified times.
///
/// Does not include any data on users without either a banned, or non-verified time.
///
/// ## Example endpoints:
///  - **Default**           
///     - `/api/v1/admin/banned_stats`
///
/// Makes a call to the underlying [Admin::get_user_banned_time_stats]
///
/// ## Example JSON output
/// ```json
///[
///     {
///         "profile_number": "76561198039230536",
///         "user_name": "Zypeh",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/f9/f934276c99d0f970fdcb2d4e1229dde02d778d99_full.jpg",
///         "total_runs": 1954,
///         "banned_runs": 64,
///         "non_verified_runs": 1
///     },...]
/// ```
#[get("/admin/banned_stats")]
pub async fn admin_banned_stats(pool: web::Data<PgPool>) -> Result<impl Responder> {
    Ok(web::Json(
        Admin::get_user_banned_time_stats(pool.get_ref()).await?,
    ))
}

/// **GET** method that returns lists of admins
///
/// ## Parameters:
/// - `admin_level`           
///     - **Optional** - `i32` : The level of admin the user has
///
/// ### Usage:
/// - `admin_level` = 0     
///     - Non-admin user
/// - `admin_level` = 1
///     - **DEFAULT** - Standard admin
/// - `admin_level` = 2
///     - Shadow admin - Has admin permissions, is not publically listed (Typically reserved for former admins, trusted players).
/// - `admin_level` = 3
///     - Developer admin - Has admin permissions as an active developer only
///
/// ## Example endpoints:
///  - **Default**           
///     - `/api/v1/admins`
///  - **With parameters**   
///     - `/api/v1/admins?admin-level=2`
///
/// Makes a call to the underlying [Users::get_all_admins]
///
/// ## Example JSON output
/// ```json
/// [
///     {
///         "profile_number": "76561198040982247",
///         "user_name": "Daniel",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg"
///     },
///     {
///         "profile_number": "76561198045074889",
///         "user_name": "『 Jonese1234 』",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/9f/9f6de098624a8f81ba6d2318e1923bdda1c710d6_full.jpg"
///     },...]
/// ```
#[get("/admins")]
pub async fn admins_list(
    pool: web::Data<PgPool>,
    admin_level: web::Query<AdminLevel>,
) -> Result<impl Responder> {
    Ok(web::Json(
        Users::get_all_admins(
            pool.get_ref(),
            admin_level.into_inner().admin_level.unwrap_or(1),
        )
        .await?,
    ))
}
