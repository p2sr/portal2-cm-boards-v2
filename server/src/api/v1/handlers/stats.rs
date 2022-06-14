use crate::{models::changelog::*, models::stats::*, tools::error::Result};
use actix_web::{get, web, Responder};
use sqlx::PgPool;

/// **GET** method to query for the number of scores per-user across all maps.
///
/// Returns an ordered list of valid, non-banned users with the number of valid scores that they have on the boards.
///
/// ## Example endpoint:
///  - **Default**           
///     - `/api/v1/stats/count_scores`
///
/// Makes a call to the underlying [NumScores::most_cl_enries_overall]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "count": 1925,
///         "profile_number": "76561198039230536",
///         "user_name": "Zypeh",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/dc/dc4c1cfa8f0c5b0c85354825c7711f60c3714a41_full.jpg"
///     },
///     {
///         "count": 1809,
///         "profile_number": "76561198068358920",
///         "user_name": "Kendal",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/b8/b8800fc58ceede98b32949b802aed05c8c2d9ece_full.jpg"
///     },...]
/// ```
#[get("/stats/count_scores")]
pub async fn count_scores(pool: web::Data<PgPool>) -> Result<impl Responder> {
    Ok(web::Json(
        NumScores::most_cl_enries_overall(pool.get_ref()).await?,
    ))
}

/// **GET** method to query for number of scores per-user on a specific map.
///
/// Returns an ordered list of valid, non-banned users with the number of valid scores
/// that they have on the given map_id.
///
/// ## Example endpoint:
///  - **Default**           
///     - `/api/v1/stats/count_scores/47763`
///
/// Makes a call to the underlying [NumScores::most_cl_entries_by_map]
///
/// ## Example JSON output
///
/// ```json
/// [
///     {
///         "count": 19,
///         "profile_number": "76561198124459214",
///         "user_name": "Dzhessi",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/0a/0ae75ea43933cc981e65b6562188544fc42ceba1_full.jpg"
///     },
///     {
///         "count": 17,
///         "profile_number": "76561198048179892",
///         "user_name": "Betsruner",
///         "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/e7/e7a266be79a99e3bd758c4ae8f1d386309b57a9a_full.jpg"
///     },...]
/// ```
#[get("/stats/count_scores/{map_id}")]
pub async fn count_scores_by_map(
    pool: web::Data<PgPool>,
    map_id: web::Path<String>,
) -> Result<impl Responder> {
    Ok(web::Json(
        NumScores::most_cl_entries_by_map(pool.get_ref(), &map_id).await?,
    ))
}

#[derive(Deserialize, Clone, Debug)]
pub struct LimitQuery {
    pub limit: Option<i32>,
}

/// **GET** method to generate an activity recap for the boards.
///
/// Recap based on the format created by NeKz for the p2sr Discord.
///
/// ## Example endpoint:
///  - **Default**
///     - `/api/v1/stats/recap`
/// - **Specified Limit**
///     - `/api/v1/stats/recap?limit=3`
///
/// Makes a call to the underlying [Recap::collect_recap]
///
/// ## Example JSON output:
///
/// ```json
/// {
///     "num_wrs": [
///         {
///             "profile_number": "76561198795823814",
///             "user_name": "Royal",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/d8/d84366b1be1f0439b0edc7fc8404fe2ea29a9c54_full.jpg",
///             "count": 2
///         }
///     ],
///     "num_demos": [
///         {
///             "profile_number": "76561198902321340",
///             "user_name": "Leve",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/7a/7a56621890546d1a54d4b583198b4d30411950b2_full.jpg",
///             "count": 47
///         }
///     ],
///     "top_wr_diff": [
///         {
///             "profile_number": "76561198081168311",
///             "user_name": "unity",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/39/3948dd3ae4d21772c845d4b3416bc7110b5aafb1_full.jpg",
///             "score_delta": -1138,
///             "map_id": "47845",
///             "map_name": "Industrial Fan"
///         }
///     ],
///     "most_updates": [
///         {
///             "profile_number": "76561198902321340",
///             "user_name": "Leve",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/7a/7a56621890546d1a54d4b583198b4d30411950b2_full.jpg",
///             "count": 47
///         }
///     ],
///     "top_videos": [
///         {
///             "profile_number": "76561198384377251",
///             "user_name": "Schwi",
///             "avatar": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/1b/1bb40c9b85773e89a2074fdf57ab8ab9dea1e744_full.jpg",
///             "count": 1
///         }
///     ],
///     "top_score_by_map": [
///         {
///             "map_id": "47458",
///             "map_name": "Portal Gun",
///             "count": 23
///         }
///     ]
/// }
/// ```
#[get("/stats/recap")]
pub async fn recap(
    pool: web::Data<PgPool>,
    query: web::Query<LimitQuery>,
) -> Result<impl Responder> {
    Ok(web::Json(
        Recap::collect_recap(pool.get_ref(), query.into_inner().limit).await?,
    ))
}

#[get("/stats/badges")]
pub async fn badges(pool: web::Data<PgPool>) -> Result<impl Responder> {
    Ok(web::Json(Badges::get_bages(pool.get_ref()).await?))
}

#[get("/stats/badges/{profile_number}")]
pub async fn users_badges(
    pool: web::Data<PgPool>,
    profile_number: web::Path<String>,
) -> Result<impl Responder> {
    Ok(web::Json(
        BadgeEntries::get_badge_by_user(pool.get_ref(), &profile_number).await?,
    ))
}
