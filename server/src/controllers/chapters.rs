use crate::models::chapters::{ChapterQueryParams, Chapters, Games};
use sqlx::PgPool;

impl Chapters {
    /// Returns the maps for a given chapter.
    pub async fn get_map_ids(pool: &PgPool, chapter_id: i32) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT maps.steam_id FROM maps WHERE chapter_id=$1"#)
            .bind(chapter_id)
            .fetch_all(pool)
            .await
    }
    /// Returns a chapter's data by the ID given.
    pub async fn get_chapter_by_id(pool: &PgPool, chapter_id: i32) -> Result<Option<Chapters>, sqlx::Error> {
        sqlx::query_as::<_, Chapters>(r#"SELECT * FROM chapters WHERE id=$1;"#)
            .bind(chapter_id)
            .fetch_optional(pool)
            .await
    }
    #[allow(dead_code)]
    /// Returns true if the map is multiplayer, false if the map is singleplayer
    pub async fn get_chapter_is_multiplayer(
        pool: &PgPool,
        chapter_id: i32,
    ) -> Result<Option<bool>, sqlx::Error> {
        sqlx::query_scalar(r#"SELECT is_multiplayer FROM chapters WHERE id=$1"#)
            .bind(chapter_id)
            .fetch_optional(pool)
            .await
    }
    #[allow(dead_code)]
    pub async fn get_chapter_game(pool: &PgPool, chapter_id: i32) -> Result<Option<Games>, sqlx::Error> {
        sqlx::query_as::<_, Games>(
            r#"SELECT games.id, games.game_name 
            FROM games
            INNER JOIN chapters ON (games.id = chapters.game_id)
            WHERE chapters.id = $1"#,
        )
            .bind(chapter_id)
            .fetch_optional(pool)
            .await
    }
    pub async fn get_filtered_chapters(
        pool: &PgPool,
        params: ChapterQueryParams,
    ) -> Result<Vec<Chapters>, sqlx::Error> {
        sqlx::query_as::<_, Chapters>(&build_filtered_chapter(params).await)
            .fetch_all(pool)
            .await
    }
}

// TODO: Do we want to return a chapter/map bundled information?
///
pub async fn build_filtered_chapter(params: ChapterQueryParams) -> String {
    let mut query_string: String = String::from(r#"SELECT * FROM chapters"#);
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
    query_string
}
