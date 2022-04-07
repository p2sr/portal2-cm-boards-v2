use crate::models::models::*;
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

impl Maps {
    pub async fn get_maps(pool: &PgPool, game_id: i32) -> Result<Vec<Maps>> {
        let res = sqlx::query_as::<_, Maps>(
            r#"
            SELECT maps.id, maps.steam_id, maps.lp_id,
            maps.name, maps.chapter_id, maps.default_cat_id,
            maps.is_public  FROM "p2boards".maps 
                INNER JOIN "p2boards".chapters ON (maps.chapter_id = chapters.id)
                WHERE chapters.game_id = $1;"#,
        )
        .bind(game_id)
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
    /// Takes in a bool, if true returns MP map_ids, if false, returns as SP map_ids
    pub async fn get_steam_ids(pool: &PgPool, is_mp: bool) -> Result<Vec<String>> {
        let res = sqlx::query(
            r#"
                SELECT maps.steam_id FROM "p2boards".maps
                    INNER JOIN "p2boards".chapters ON (maps.chapter_id = chapters.id)
                    WHERE chapters.is_multiplayer = $1"#,
        )
        .bind(is_mp)
        .map(|row: PgRow| row.get(0))
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
    pub async fn get_steam_ids_by_game(pool: &PgPool, game: i32) -> Result<Vec<String>> {
        let res = sqlx::query(
            r#"
                SELECT maps.steam_id FROM "p2boards".maps
                    INNER JOIN "p2boards".chapters ON (maps.chapter_id = chapters.id)
                    INNER JOIN "p2boards".games ON (chapter.game_id = games.id)
                    WHERE games.id = $1"#,
        )
        .bind(game)
        .map(|row: PgRow| row.get(0))
        .fetch_all(pool)
        .await?;
        Ok(res)
    }
    /// Returns the map name for a given steam_id.
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>> {
        let res = sqlx::query(r#"SELECT maps.name FROM "p2boards".maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .map(|row: PgRow| row.get(0))
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns all default cats
    pub async fn get_all_default_cats(pool: &PgPool) -> Result<HashMap<String, i32>> {
        let mut hm: HashMap<String, i32> = HashMap::with_capacity(108);
        sqlx::query(r#"SELECT steam_id, default_cat_id FROM "p2boards".maps"#)
            .map(|row: PgRow| hm.insert(row.get(0), row.get(1)))
            .fetch_all(pool)
            .await?;
        Ok(hm)
    }
    /// Returns the default category for a given map.
    pub async fn get_default_cat(pool: &PgPool, map_id: String) -> Result<Option<i32>> {
        let res = sqlx::query(
            r#"
                SELECT default_cat_id FROM "p2boards".maps
                WHERE steam_id = $1;"#,
        )
        .bind(map_id)
        .map(|row: PgRow| row.get(0))
        .fetch_one(pool)
        .await?;
        Ok(res)
    }
    /// Returns chapter information for a given map_id (steam_id)
    #[allow(dead_code)]
    pub async fn get_chapter_from_map_id(
        pool: &PgPool,
        map_id: String,
    ) -> Result<Option<Chapters>> {
        let res = sqlx::query_as::<_, Chapters>(
            r#"
                SELECT chapters.id, chapters.chapter_name, chapters.is_multiplayer, chapters.game_id
                FROM "p2boards".Chapters
                INNER JOIN "p2boards".maps ON (chapters.id = maps.chapter_id)
                WHERE maps.steam_id = $1"#,
        )
        .bind(map_id)
        .fetch_one(pool)
        .await?;
        Ok(Some(res))
    }
    /// Searches for all chapter IDs that match a given search pattern.
    #[allow(dead_code)]
    pub async fn get_steam_id_by_name(
        pool: &PgPool,
        map_name: String,
    ) -> Result<Option<Vec<String>>> {
        let query_map_name = format!("%{}%", &map_name);
        let res = sqlx::query(
            r#"SELECT steam_id FROM "p2boards".maps 
                WHERE LOWER(name) LIKE LOWER($1)"#,
        )
        .bind(query_map_name)
        .map(|row: PgRow| row.get(0))
        .fetch_all(pool)
        .await?;
        Ok(Some(res))
    }
    /// Returns true if the map is publicly accessible on the Steam Leaderboards.
    #[allow(dead_code)]
    pub async fn get_is_public_by_steam_id(pool: &PgPool, map_id: String) -> Result<Option<bool>> {
        let res = sqlx::query(r#"SELECT is_public FROM "p2boards".maps WHERE steam_id = $1;"#)
            .bind(map_id)
            .map(|row: PgRow| row.get(0))
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
}
