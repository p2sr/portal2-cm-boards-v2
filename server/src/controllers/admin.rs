use crate::controllers::changelog::build_filtered_changelog;
use crate::models::models::*;
use anyhow::{bail, Result};
use sqlx::PgPool;

impl Admin {
    pub async fn get_admin_page(
        pool: &PgPool,
        params: ChangelogQueryParams,
    ) -> Result<Option<Vec<ChangelogPage>>> {
        // TODO: Add a ratio of verified/non-verified scores, # banned per-user.
        let mut additional_filters: Vec<String> =
            vec!["cl.banned = 'true' OR cl.verified = 'false' OR u.banned = 'true'".to_string()];
        let query_string =
            match build_filtered_changelog(pool, params, Some(&mut additional_filters)).await {
                Ok(s) => s,
                Err(e) => bail!(e),
            };
        println!("{}", query_string);
        let res = sqlx::query_as::<_, ChangelogPage>(&query_string)
            .fetch_all(pool)
            .await;
        match res {
            Ok(changelog_filtered) => Ok(Some(changelog_filtered)),
            Err(e) => {
                eprintln!("{}", query_string);
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            }
        }
    }
    pub async fn get_user_banned_time_stats(
        pool: &PgPool,
    ) -> Result<Option<Vec<BannedTimeDetails>>> {
        // NOTE: This query definitely doesn't work
        // TODO: Goal is to get the user's information, # times, and # banned times from all users with banned times.
        // TODO: Get verified % ?
        let res: Vec<BannedTimeDetails> = sqlx::query_as::<_, BannedTimeDetails>(
            r#"
            SELECT 
                CASE
                    WHEN users.board_name IS NULL
                        THEN users.steam_name
                    WHEN users.board_name IS NOT NULL
                        THEN users.board_name
                    END user_name, users.avatar,
                    (SELECT COUNT(*) FROM "p2boards".changelog WHERE profile_number = users.profile_number) as num_times,
                    (SELECT COUNT(*) FROM "p2boards".changelog WHERE profile_number = users.profile_number AND banned = 'true') as times_banned
                FROM "p2boards".users
                INNER JOIN "p2boards".changelog ON (users.profile_number = changelog.profile_number)
                WHERE "#)
            .fetch_all(pool)
            .await
            .unwrap();

        Ok(Some(vec![]))
    }
}
