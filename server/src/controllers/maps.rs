use crate::models::chapters::*;
use crate::models::maps::*;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

impl Maps {
    /// Return all [Maps] on a given `game_id`.
    pub async fn get_maps(pool: &PgPool, game_id: i32) -> Result<Vec<Maps>, sqlx::Error> {
        sqlx::query_as::<_, Maps>(
            r#"
            SELECT maps.* FROM maps 
                INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                WHERE chapters.game_id = $1;"#,
        )
        .bind(game_id)
        .fetch_all(pool)
        .await
    }
    /// `is_mp
    /// - If `true`
    ///     - Returns multiplayer `map_ids`.
    /// - If `false`
    ///     - Returns all singleplayer `map_ids`.
    pub async fn get_steam_ids(pool: &PgPool, is_mp: bool) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar(
            r#"
                SELECT maps.steam_id FROM maps
                    INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                    WHERE chapters.is_multiplayer = $1"#,
        )
        .bind(is_mp)
        .fetch_all(pool)
        .await
    }
    /// Returns the map `name` for a given `steam_id`.
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT maps.name FROM maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .fetch_optional(pool)
            .await
    }
    /// Returns all default categories in the game as a `HashMap` of `String` -> `i32` (`map_id` -> `cat_id`).
    pub async fn get_all_default_cats(pool: &PgPool) -> Result<HashMap<String, i32>, sqlx::Error> {
        let mut hm: HashMap<String, i32> = HashMap::with_capacity(108);
        sqlx::query(r#"SELECT steam_id, default_cat_id FROM maps"#)
            .map(|row: PgRow| hm.insert(row.get(0), row.get(1)))
            .fetch_all(pool)
            .await?;
        Ok(hm)
    }
    /// Returns the default category for a given `map_id`.
    pub async fn get_default_cat(pool: &PgPool, map_id: String) -> Result<Option<i32>, sqlx::Error> {
        sqlx::query_scalar(
            r#"
                SELECT default_cat_id FROM maps
                WHERE steam_id = $1;"#,
        )
        .bind(map_id)
        .fetch_optional(pool)
        .await
    }
    /// Returns a [Chapters] for a given `map_id`.
    #[allow(dead_code)]
    pub async fn get_chapter_from_map_id(
        pool: &PgPool,
        map_id: String,
    ) -> Result<Option<Chapters>, sqlx::Error> {
        sqlx::query_as::<_, Chapters>(
            r#"
                SELECT chapters.* FROM Chapters
                    INNER JOIN maps ON (chapters.id = maps.chapter_id)
                    WHERE maps.steam_id = $1"#,
        )
        .bind(map_id)
        .fetch_optional(pool)
        .await
    }
    /// Searches for all `chapter_id`s by a given `name` value. 
    /// 
    /// Designed to be as easy to work with as possible for site-searches.
    #[allow(dead_code)]
    pub async fn get_steam_id_by_name(
        pool: &PgPool,
        map_name: String,
    ) -> Result<Vec<String>, sqlx::Error> {
        // Limitation to how SQLX inserts strings.
        let query_map_name = format!("%{}%", &map_name);
        sqlx::query_scalar(
            r#"SELECT steam_id FROM maps 
                WHERE LOWER(name) LIKE LOWER($1)"#,
        )
        .bind(query_map_name)
        .fetch_all(pool)
        .await
    }
    /// Returns `true` if the `map_id` is publicly accessible on the Steam Leaderboards.
    #[allow(dead_code)]
    pub async fn get_is_public_by_steam_id(pool: &PgPool, map_id: String) -> Result<Option<bool>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT is_public FROM maps WHERE steam_id = $1;"#)
            .bind(map_id)
            .fetch_optional(pool)
            .await
    }
}
