use crate::models::chapters::*;
use crate::models::maps::*;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

impl Maps {
    pub async fn get_maps(pool: &PgPool, game_id: i32) -> Result<Vec<Maps>, sqlx::Error> {
        sqlx::query_as::<_, Maps>(
            r#"
            SELECT maps.id, maps.steam_id, maps.lp_id,
            maps.name, maps.chapter_id, maps.default_cat_id,
            maps.is_public  FROM maps 
                INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                WHERE chapters.game_id = $1;"#,
        )
        .bind(game_id)
        .fetch_all(pool)
        .await
    }
    /// Takes in a bool, if true returns MP map_ids, if false, returns as SP map_ids
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
    #[allow(dead_code)]
    pub async fn get_steam_ids_by_game(pool: &PgPool, game: i32) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar(
            r#"
                SELECT maps.steam_id FROM maps
                    INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                    INNER JOIN games ON (chapter.game_id = games.id)
                    WHERE games.id = $1"#,
        )
        .bind(game)
        .fetch_all(pool)
        .await
    }
    /// Returns the map name for a given steam_id.
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT maps.name FROM maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .fetch_optional(pool)
            .await
    }
    /// Returns all default cats
    pub async fn get_all_default_cats(pool: &PgPool) -> Result<HashMap<String, i32>, sqlx::Error> {
        let mut hm: HashMap<String, i32> = HashMap::with_capacity(108);
        sqlx::query(r#"SELECT steam_id, default_cat_id FROM maps"#)
            .map(|row: PgRow| hm.insert(row.get(0), row.get(1)))
            .fetch_all(pool)
            .await?;
        Ok(hm)
    }
    /// Returns the default category for a given map.
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
    /// Returns chapter information for a given map_id (steam_id)
    #[allow(dead_code)]
    pub async fn get_chapter_from_map_id(
        pool: &PgPool,
        map_id: String,
    ) -> Result<Option<Chapters>, sqlx::Error> {
        sqlx::query_as::<_, Chapters>(
            r#"
                SELECT chapters.id, chapters.chapter_name, chapters.is_multiplayer, chapters.game_id
                FROM Chapters
                INNER JOIN maps ON (chapters.id = maps.chapter_id)
                WHERE maps.steam_id = $1"#,
        )
        .bind(map_id)
        .fetch_optional(pool)
        .await
    }
    /// Searches for all chapter IDs that match a given search pattern.
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
    /// Returns true if the map is publicly accessible on the Steam Leaderboards.
    #[allow(dead_code)]
    pub async fn get_is_public_by_steam_id(pool: &PgPool, map_id: String) -> Result<Option<bool>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT is_public FROM maps WHERE steam_id = $1;"#)
            .bind(map_id)
            .fetch_optional(pool)
            .await
    }
}
