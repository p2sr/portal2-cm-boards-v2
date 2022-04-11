use crate::models::models::{ChapterQueryParams, Chapters, Games};
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

impl Chapters {
    /// Returns the maps for a given chapter.
    pub async fn get_map_ids(pool: &PgPool, chapter_id: i32) -> Result<Option<Vec<String>>> {
        Ok(Some(
            sqlx::query(r#"SELECT maps.steam_id FROM p2boards.maps WHERE chapter_id=$1"#)
                .bind(chapter_id)
                .map(|row: PgRow| row.get(0))
                .fetch_all(pool)
                .await?,
        ))
    }
    // /// Searches for all chapters that match a given search pattern.
    // pub async fn get_chapter_by_name(
    //     pool: &PgPool,
    //     chapter_name: String,
    // ) -> Result<Option<Vec<Chapters>>> {
    //     let query_chapter_name = format!("%{}%", &chapter_name);
    //     let res = sqlx::query_as::<_, Chapters>(
    //         r#"SELECT * FROM "p2boards".chapters
    //             WHERE LOWER(chapter_name) LIKE LOWER(%$1%)"#,
    //     )
    //     .bind(query_chapter_name)
    //     .fetch_all(pool)
    //     .await?;
    //     Ok(Some(res))
    // }
    /// Returns a chapter's data by the ID given.
    #[allow(dead_code)]
    pub async fn get_chapter_by_id(pool: &PgPool, chapter_id: i32) -> Result<Option<Chapters>> {
        Ok(Some(
            sqlx::query_as::<_, Chapters>(r#"SELECT * FROM "p2boards".chapters WHERE id=$1;"#)
                .bind(chapter_id)
                .fetch_one(pool)
                .await?,
        ))
    }
    /// Returns true if the map is multiplayer, false if the map is singleplayer
    #[allow(dead_code)]
    pub async fn get_chapter_is_multiplayer(
        pool: &PgPool,
        chapter_id: i32,
    ) -> Result<Option<bool>> {
        Ok(Some(
            sqlx::query(r#"SELECT is_multiplayer FROM "p2boards".chapters WHERE id=$1"#)
                .bind(chapter_id)
                .map(|row: PgRow| row.get(0))
                .fetch_one(pool)
                .await?,
        ))
    }
    #[allow(dead_code)]
    pub async fn get_chapter_game(pool: &PgPool, chapter_id: i32) -> Result<Option<Games>> {
        Ok(Some(
            sqlx::query_as::<_, Games>(
                r#"SELECT games.id, games.game_name 
                FROM "p2boards".games
                INNER JOIN "p2boards".chapters ON (games.id = chapters.game_id)
                WHERE chapters.id = $1"#,
            )
            .bind(chapter_id)
            .fetch_one(pool)
            .await?,
        ))
    }
    pub async fn get_filtered_chapters(
        pool: &PgPool,
        params: ChapterQueryParams,
    ) -> Result<Option<Vec<Chapters>>> {
        let query_string = build_filtered_chapter(params).await?;
        Ok(Some(
            sqlx::query_as::<_, Chapters>(&query_string)
                .fetch_all(pool)
                .await?,
        ))
    }
}

// TODO: Do we want to return a chapter/map bundled information?
///
pub async fn build_filtered_chapter(params: ChapterQueryParams) -> Result<String> {
    let mut query_string: String = String::from(r#"SELECT * FROM "p2boards".chapters"#);
    let mut filters: Vec<String> = Vec::new();
    if let Some(chapter_name) = params.chapter_name {
        filters.push(format!(
            "LOWER(chapter_name) LIKE LOWER('%{}%')\n",
            chapter_name
        ));
    }
    if let Some(is_multiplayer) = params.is_multiplayer {
        match is_multiplayer {
            true => filters.push("is_multiplayer = 'true'\n".to_string()),
            false => filters.push("is_multiplayer = 'false'\n".to_string()),
        }
    }
    if let Some(game_id) = params.game_id {
        filters.push(format!("game_id = {}\n", game_id));
    }
    // Build the statement based off the elements we added to our vector (used to make sure only first statement is WHERE, and additional are OR)
    for (i, entry) in filters.iter().enumerate() {
        match i {
            0 => query_string = format!("{} WHERE {}", query_string, entry),
            _ => query_string = format!("{} AND {}", query_string, entry),
        }
    }
    Ok(query_string)
}
