use crate::models::changelog::*;
use crate::tools::error::Result;
use actix_web::{get, web, Responder};
use sqlx::PgPool;

#[get("/stats/count_scores")]
pub async fn count_scores(pool: web::Data<PgPool>) -> Result<impl Responder> {
    Ok(web::Json(
        NumScores::most_cl_enries_overall(pool.get_ref()).await?,
    ))
}

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

#[get("/recap")]
pub async fn recap(
    pool: web::Data<PgPool>,
    query: web::Query<LimitQuery>,
) -> Result<impl Responder> {
    Ok(web::Json(
        Recap::collect_recap(pool.get_ref(), query.into_inner().limit).await?,
    ))
}
