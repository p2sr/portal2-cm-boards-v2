use crate::models::{maps::Maps, sp::*};

use futures::future::try_join_all;
use sqlx::PgPool;

impl SpMap {
    /// Returns a Single Player Map Page.
    /// 
    /// ### Params
    /// - `map_id`
    ///     - The map returned.
    /// - `limit`
    ///     - The max number of entries returned.
    /// - `cat_id`
    ///     - The category the runs have to be from.
    /// - `game_id`
    ///     - The game thee runs have to be from.
    pub async fn get_sp_map_page(
        pool: &PgPool,
        map_id: &String,
        limit: i32,
        cat_id: i32,
        game_id: i32,
    ) -> Result<Vec<SpMap>, sqlx::Error> {
        sqlx::query_as::<_, SpMap>(
            r#" 
                SELECT t.timestamp,
                    t.CL_profile_number,
                    t.score,
                    t.demo_id,
                    t.youtube_id,
                    t.submission,
                    t.note,
                    t.category_id,
                    COALESCE(t.board_name, t.steam_name) AS user_name,
                    t.avatar
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM changelog
                    INNER JOIN users ON (users.profile_number = changelog.profile_number)
                    INNER JOIN maps ON (changelog.map_id = maps.steam_id)
                    INNER JOIN chapters ON (maps.chapter_id = chapters.id)
                        WHERE map_id = $1
                        AND users.banned = False
                        AND changelog.verified = True
                        AND changelog.banned = False
                        AND changelog.category_id = $2
                        AND chapters.game_id = $3
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score
                LIMIT $3"#,
        )
        .bind(map_id)
        .bind(cat_id)
        .bind(game_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}

impl SpPreview {
    /// Gets preview information for top 7 on an SP Map.
    pub async fn get_sp_preview(pool: &PgPool, map_id: &str) -> Result<Vec<SpPreview>, sqlx::Error> {
        sqlx::query_as::<_, SpPreview>(
            r#"
                SELECT t.CL_profile_number, t.score, t.youtube_id, t.category_id,
                COALESCE(t.board_name, t.steam_name) AS user_name, t.map_id
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM changelog
                    INNER JOIN users ON (users.profile_number = changelog.profile_number)
                    WHERE map_id = $1
                    AND users.banned = False
                    AND changelog.banned = False
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
               ORDER BY score
               LIMIT 7;"#,
        )
        .bind(map_id)
        .fetch_all(pool)
        .await
    }
    /// Collects the top 7 preview data for all SP maps.
    pub async fn get_sp_previews(pool: &PgPool) -> Result<Vec<Vec<SpPreview>>, sqlx::Error> {
        let map_id_vec = Maps::get_steam_ids(pool, false).await?;
        let futures: Vec<_> = map_id_vec
            .iter()
            .map(|map_id| SpPreview::get_sp_preview(pool, map_id))
            .collect();
        try_join_all(futures).await
    }
}

impl SpBanned {
    /// Returns all profile_numbers and scores associated with banned times on a given map
    pub async fn get_sp_banned(pool: &PgPool, map_id: String) -> Result<Vec<SpBanned>, sqlx::Error> {
        sqlx::query_as::<_, SpBanned>(
            r#"
                SELECT changelog.profile_number, changelog.score 
                    FROM changelog
                    WHERE changelog.banned = True
                        AND changelog.map_id = $1
                    ORDER BY changelog.score ASC
            "#,
        )
        .bind(map_id)
        .fetch_all(pool)
        .await
    }
}
