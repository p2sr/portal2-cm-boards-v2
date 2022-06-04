use crate::models::changelog::*;
use crate::models::users::UsersDisplayCount;
use sqlx::PgPool;

impl NumScores {
    pub async fn most_cl_enries_overall(pool: &PgPool) -> Result<Vec<NumScores>, sqlx::Error> {
        sqlx::query_as::<_, NumScores>(r#"SELECT COUNT(*), changelog.profile_number, COALESCE(board_name, steam_name) AS user_name, avatar
            FROM changelog INNER JOIN users ON (users.profile_number = changelog.profile_number)
            WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
        GROUP BY changelog.profile_number, user_name, avatar
        ORDER BY COUNT(*) DESC;"#)
        .fetch_all(pool)
        .await
    }
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

impl Recap {
    pub async fn get_num_wrs(pool: &PgPool) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE post_rank = 1 AND users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC;"#)
        .fetch_all(pool)
        .await
    }
    pub async fn get_num_demos(pool: &PgPool) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number, 
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE demo_id IS NOT NULL AND users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC;"#)
        .fetch_all(pool)
        .await
    }
    pub async fn get_top_pb_diff(pool: &PgPool) -> Result<Vec<ScoreDeltaComparison>, sqlx::Error> {
        sqlx::query_as::<_, ScoreDeltaComparison>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, score_delta, map_id, maps.name AS map_name 
            FROM changelog
            INNER JOIN users ON (changelog.profile_number = users.profile_number)
            INNER JOIN maps ON (changelog.map_id = maps.steam_id)
                WHERE score_delta IS NOT NULL AND users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '30 days'
            GROUP BY changelog.profile_number, user_name, avatar, score_delta, map_id, map_name ORDER BY score_delta ASC;"#)
        .fetch_all(pool)
        .await
    }
    pub async fn get_most_updates(pool: &PgPool) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC;"#)
        .fetch_all(pool)
        .await
    }
    pub async fn get_top_videos(pool: &PgPool) -> Result<Vec<UsersDisplayCount>, sqlx::Error> {
        sqlx::query_as::<_, UsersDisplayCount>(
            r#"SELECT changelog.profile_number,
        COALESCE(board_name, steam_name) AS user_name, avatar, COUNT(*) AS count
            FROM changelog INNER JOIN users ON (changelog.profile_number = users.profile_number)
                WHERE youtube_id IS NOT NULL AND users.banned = false AND changelog.banned = false
                AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY changelog.profile_number, user_name, avatar ORDER BY COUNT(*) DESC;"#,
        )
        .fetch_all(pool)
        .await
    }
    pub async fn get_top_wrs_by_map(pool: &PgPool) -> Result<Vec<NumWrsPerMap>, sqlx::Error> {
        sqlx::query_as::<_, NumWrsPerMap>(r#"SELECT map_id, maps.name AS map_name, COUNT(*) AS count
            FROM changelog
            INNER JOIN users ON (changelog.profile_number = users.profile_number)
            INNER JOIN maps ON (maps.steam_id = changelog.map_id)
                WHERE users.banned = false AND changelog.banned = false AND changelog.verified = true
                AND timestamp > current_date - interval '7 days'
            GROUP BY map_id, map_name ORDER BY count DESC;"#)
        .fetch_all(pool)
        .await
    }
    pub async fn collect_recap(pool: &PgPool) -> Result<Recap, sqlx::Error> {
        Ok(Recap {
            num_wrs: Recap::get_num_wrs(pool).await?,
            num_demos: Recap::get_num_demos(pool).await?,
            top_pb_diff: Recap::get_top_pb_diff(pool).await?,
            most_updates: Recap::get_most_updates(pool).await?,
            top_videos: Recap::get_top_videos(pool).await?,
            top_wrs_by_map: Recap::get_top_wrs_by_map(pool).await?,
        })
    }
}
