use crate::models::models::*;
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

impl Chapters {
    /// Returns the maps for a given chapter.
    pub async fn get_map_ids(pool: &PgPool, chapter_id: i32) -> Result<Option<Vec<String>>> {
        let res = sqlx::query(r#"SELECT maps.steam_id FROM p2boards.maps WHERE chapter_id=$1"#)
            .bind(chapter_id)
            .map(|row: PgRow| row.get(0))
            .fetch_all(pool)
            .await?;
        Ok(Some(res)) //We're not going to handle error cases I'm tired
                      // TODO: Do this better
    }
    /// Searches for all chapters that match a given search pattern.
    pub async fn get_chapter_by_name(
        pool: &PgPool,
        chapter_name: String,
    ) -> Result<Option<Vec<Chapters>>> {
        let query_chapter_name = format!("%{}%", &chapter_name);
        let res = sqlx::query_as::<_, Chapters>(
            r#"SELECT * FROM "p2boards".chapters 
                WHERE LOWER(chapter_name) LIKE LOWER($1)"#,
        )
        .bind(query_chapter_name)
        .fetch_all(pool)
        .await?;
        Ok(Some(res))
    }
    /// Returns a chapter's data by the ID given.
    #[allow(dead_code)]
    pub async fn get_chapter_by_id(pool: &PgPool, chapter_id: i32) -> Result<Option<Chapters>> {
        let res =
            sqlx::query_as::<_, Chapters>(r#"SELECT * FROM "p2boards".chapters WHERE id=$1;"#)
                .bind(chapter_id)
                .fetch_one(pool)
                .await?;
        Ok(Some(res))
    }
    /// Returns true if the map is multiplayer, false if the map is singleplayer
    #[allow(dead_code)]
    pub async fn get_chapter_is_multiplayer(
        pool: &PgPool,
        chapter_id: i32,
    ) -> Result<Option<bool>> {
        let res = sqlx::query(r#"SELECT is_multiplayer FROM "p2boards".chapters WHERE id=$1"#)
            .bind(chapter_id)
            .map(|row: PgRow| row.get(0))
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    #[allow(dead_code)]
    pub async fn get_chapter_game(pool: &PgPool, chapter_id: i32) -> Result<Option<Games>> {
        let res = sqlx::query_as::<_, Games>(
            r#"SELECT games.id, games.game_name 
                FROM "p2boards".games
                INNER JOIN "p2boards".chapters ON (games.id = chapters.game_id)
                WHERE chapters.id = $1"#,
        )
        .bind(chapter_id)
        .fetch_one(pool)
        .await?;
        Ok(Some(res))
    }
}
