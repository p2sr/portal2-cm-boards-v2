use crate::models::{changelog::*, stats::*, users::UsersDisplayCount};
use sqlx::PgPool;

impl NumScores {
    /// Returns a Vec of [NumScores] for total number of valid changelog entries across the entire boards.
    pub async fn most_cl_enries_overall(pool: &PgPool) -> Result<Vec<NumScores>, sqlx::Error> {
        sqlx::query_as::<_, NumScores>(r#"SELECT COUNT(*), changelog.profile_number, COALESCE(board_name, steam_name) AS user_name, avatar
            FROM changelog INNER JOIN users ON (users.profile_number = changelog.profile_number)
            WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
        GROUP BY changelog.profile_number, user_name, avatar
        ORDER BY COUNT(*) DESC;"#)
        .fetch_all(pool)
        .await
    }
    // TODO: game_id/cat_id.
    /// Returns a Vec of [NumScores] for total number of valid changelog entries for a given map.
    pub async fn most_cl_entries_by_map(
        pool: &PgPool,
        map_id: &str,
    ) -> Result<Vec<NumScores>, sqlx::Error> {
        sqlx::query_as::<_, NumScores>(r#"SELECT COUNT(*), changelog.profile_number, COALESCE(board_name, steam_name) AS user_name, avatar
            FROM changelog
            INNER JOIN users ON (users.profile_number = changelog.profile_number)
            INNER JOIN maps ON (changelog.map_id = maps.steam_id)
            WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
            AND map_id = $1
        GROUP BY changelog.profile_number, user_name, avatar
        ORDER BY COUNT(*) DESC;"#)
        .bind(map_id)
        .fetch_all(pool)
        .await
    }
}

// TODO: Allow changing the day interval.
impl Recap {
    /// Returns a Vec of [UsersDisplayCount] to display the users with the most WRs for the given time period.
    pub async fn get_num_wrs(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE post_rank = 1 AND users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC LIMIT $1;"#)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    /// Returns a Vec of [UsersDisplayCount] to display the users who have the most demos for the given time period.
    pub async fn get_num_demos(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number, 
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE demo_id IS NOT NULL AND users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC LIMIT $1;"#)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    // Note: This is left to treat SP/Coop as the same because nothing gaurentees that the score delta
    // for both players on a coop time will be the same, so we treat these like single entries, even if coop
    // entries share a score delta.
    /// Returns a Vec of [ScoreDeltaComparison] to display the users who have the largest score deltas in a given time period.
    pub async fn get_top_wr_diff(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<ScoreDeltaComparison>, sqlx::Error> {
        sqlx::query_as::<_, ScoreDeltaComparison>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, score_delta, map_id, maps.name AS map_name 
            FROM changelog
            INNER JOIN users ON (changelog.profile_number = users.profile_number)
            INNER JOIN maps ON (changelog.map_id = maps.steam_id)
                WHERE score_delta IS NOT NULL AND post_rank = 1 AND users.banned = false AND changelog.banned = false 
                AND changelog.verified = true AND timestamp > current_date - interval '30 days'
            GROUP BY changelog.profile_number, user_name, avatar, score_delta, map_id, map_name ORDER BY score_delta ASC LIMIT $1;"#)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    /// Returns a Vec of [UsersDisplayCount] to display the users who have the most changelog entries for the given time period.
    pub async fn get_most_updates(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC LIMIT $1;"#)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    /// Returns a Vec of [UsersDisplayCount] to display the users who have the most youtube links for the given time period.
    pub async fn get_top_videos(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(
            r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE youtube_id IS NOT NULL AND users.banned = false AND changelog.banned = false
                AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC LIMIT $1;"#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    /// Returns a Vec of [NumUpdatePerMap] to display the maps with the most changelog entries in a given time period.
    pub async fn get_top_update_by_map(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<NumUpdatePerMap>, sqlx::Error> {
        sqlx::query_as::<_, NumUpdatePerMap>(r#"SELECT map_id, maps.name AS map_name, COUNT(*) AS count
            FROM changelog
            INNER JOIN users ON (changelog.profile_number = users.profile_number)
            INNER JOIN maps ON (maps.steam_id = changelog.map_id)
                WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY map_id, map_name ORDER BY count DESC LIMIT $1;"#)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
    // TODO: Maybe there's a way to `join_all` this? The futures are different which could be c.
    /// Collection method to generate a [Recap] from all of the individual fetching methods.
    pub async fn collect_recap(pool: &PgPool, limit: Option<i32>) -> Result<Recap, sqlx::Error> {
        let limit = limit.unwrap_or(5);
        Ok(Recap {
            num_wrs: Recap::get_num_wrs(pool, limit).await?,
            num_demos: Recap::get_num_demos(pool, limit).await?,
            top_wr_diff: Recap::get_top_wr_diff(pool, limit).await?,
            most_updates: Recap::get_most_updates(pool, limit).await?,
            top_videos: Recap::get_top_videos(pool, limit).await?,
            top_score_by_map: Recap::get_top_update_by_map(pool, limit).await?,
        })
    }
}

impl Badges {
    /// Returns a vec of all [Badges] on the boards.
    pub async fn get_bages(pool: &PgPool) -> Result<Vec<Badges>, sqlx::Error> {
        sqlx::query_as::<_, Badges>(r#"SELECT * FROM badges;"#)
            .fetch_all(pool)
            .await
    }
    /// Get a badge by ID.
    pub async fn get_badge_by_id(
        pool: &PgPool,
        badge_id: i32,
    ) -> Result<Option<Badges>, sqlx::Error> {
        sqlx::query_as::<_, Badges>(r#"SELECT * FROM badges WHERE id = $1;"#)
            .bind(badge_id)
            .fetch_optional(pool)
            .await
    }
    pub async fn search_badge(pool: &PgPool, search_str: &str) -> Result<Vec<Badges>, sqlx::Error> {
        // Build in `%` for match wildcard front and back for search string.
        let search_str = format!("%{}%", &search_str);
        sqlx::query_as::<_, Badges>(r#"SELECT * FROM badges WHERE name LIKE LOWER($1);"#)
            .bind(search_str)
            .fetch_all(pool)
            .await
    }
    pub async fn insert_badge(
        pool: &PgPool,
        new_badge: BadgeInsert,
    ) -> Result<Badges, sqlx::Error> {
        sqlx::query_as::<_, Badges>(
            r#"INSERT INTO badges 
            (name, image, description, tier) VALUES ($1, $2, $3, $4)
            RETURNING *;"#,
        )
        .bind(new_badge.name)
        .bind(new_badge.image)
        .bind(new_badge.description)
        .bind(new_badge.tier)
        .fetch_one(pool)
        .await
    }
    pub async fn update_badge(pool: &PgPool, badge: Badges) -> Result<Badges, sqlx::Error> {
        sqlx::query_as::<_, Badges>(
            r#"UPDATE badges SET name = $2, image = $3, description = $4, tier = $5 WHERE id = $1 RETURNING *;"#,
        )
        .bind(badge.id)
        .fetch_one(pool)
        .await
    }
    pub async fn delete_badge(pool: &PgPool, badge_id: i32) -> Result<Badges, sqlx::Error> {
        sqlx::query_as::<_, Badges>(
            r#"DELETE FROM badges 
            WHERE id = $1 RETURNING *"#,
        )
        .bind(badge_id)
        .fetch_one(pool)
        .await
    }
}

impl BadgeEntries {
    pub async fn get_badge_by_user(
        pool: &PgPool,
        profile_number: &str,
    ) -> Result<Vec<BadgeEntries>, sqlx::Error> {
        sqlx::query_as::<_, BadgeEntries>(
            r#"SELECT * FROM badge_entries WHERE profile_number = $1;"#,
        )
        .bind(profile_number)
        .fetch_all(pool)
        .await
    }
}
