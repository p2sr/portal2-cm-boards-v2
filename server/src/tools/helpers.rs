use crate::controllers::models::Maps;
use anyhow::{bail, Result};
use num::pow;
use sqlx::PgPool;
use std::collections::HashMap;

/// Calcultes the score using the pre-existing iVerb point formula.
pub fn score(i: i32) -> f32 {
    let i = i as f32;
    let res: f32 = pow(200.0 - (i - 1.0), 2) / 200.0;
    if 1.0 > res {
        1.0
    } else {
        res
    }
}

/// Grabs the default category IDs for all maps as a HashMap.
pub async fn get_default_cat_ids(pool: &PgPool) -> HashMap<String, i32> {
    Maps::get_all_default_cats(pool).await.unwrap()
}

/// Returns the cat_id provided, or grabs it from the database.
pub async fn cat_id_check(pool: &PgPool, map_id: String, cat_id: Option<i32>) -> Result<i32> {
    let category_id: i32;
    if let Some(x) = cat_id {
        category_id = x;
    } else {
        let dcid = Maps::get_default_cat(pool, map_id.clone()).await;
        category_id = match dcid {
            Ok(Some(id)) => id,
            _ => bail!("Could not find a default cat_id for the map provided"),
        };
    }
    Ok(category_id)
}
