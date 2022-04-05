use crate::controllers::changelog::build_filtered_changelog;
use crate::models::models::*;
use anyhow::{bail, Result};
use sqlx::PgPool;

impl Admin {
    /// Returns a changelog page that filtered to information for ease of use for admins.
    ///
    /// Uses [crate::controllers::changelog::build_filtered_changelog] to build the filtered query.
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
    /// Returns a [crate::models::models::BannedTimeDetails] to display information on specific users and their problematic scores.
    pub async fn get_user_banned_time_stats(
        pool: &PgPool,
    ) -> Result<Option<Vec<BannedTimeDetails>>> {
        let res: Vec<BannedTimeDetails> = sqlx::query_as::<_, BannedTimeDetails>(
            r#"SELECT d.profile_number, d.user_name, d.avatar, d.total_runs, d.banned_runs, d.non_verified_runs
            FROM "p2boards".users
            FULL OUTER JOIN (
              SELECT users1.profile_number, 
              COALESCE(users1.board_name, users1.steam_name) AS user_name,
              users1.avatar, a.banned_runs, b.total_runs, c.non_verified_runs
                  FROM "p2boards".users AS users1
                  FULL OUTER JOIN ( 
                      SELECT usr3.profile_number, COUNT(cl2.id) AS non_verified_runs
                          FROM "p2boards".changelog as cl2
                          INNER JOIN "p2boards".users AS usr3 ON (usr3.profile_number = cl2.profile_number)
                          WHERE cl2.verified = 'false'
                          GROUP BY usr3.profile_number)
                      AS c
                      ON users1.profile_number = c.profile_number
                  FULL OUTER JOIN (
                      SELECT usr.profile_number,
                      COUNT(changelog.id) AS banned_runs
                          FROM "p2boards".changelog
                          INNER JOIN "p2boards".users AS usr ON (usr.profile_number = changelog.profile_number)
                          WHERE changelog.banned = 'true'
                          GROUP BY usr.profile_number) 
                      AS a
                      ON users1.profile_number = a.profile_number
                  FULL OUTER JOIN (
                      SELECT usr2.profile_number,
                      COUNT(cl.id) AS total_runs
                          FROM "p2boards".changelog as cl
                          INNER JOIN "p2boards".users AS usr2 ON (usr2.profile_number = cl.profile_number)
                          GROUP BY usr2.profile_number)
                      AS b
                      ON users1.profile_number = b.profile_number)
              AS d
              ON d.profile_number = users.profile_number
            WHERE d.non_verified_runs IS NOT NULL 
            OR d.banned_runs IS NOT NULL
          ORDER BY d.total_runs DESC;"#)
            .fetch_all(pool)
            .await
            .unwrap();

        Ok(Some(res))
    }
}
