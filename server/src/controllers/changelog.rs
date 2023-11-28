use std::collections::HashMap;
use sqlx::PgPool;
use chrono::NaiveDateTime;
use crate::models::changelog::*;
use crate::models::users::Users;
use crate::tools::helpers::Transaction;

// Implementations of associated functions for Changelog
impl Changelog {
    /// Search for a [Changelog] by ID, return the entire [Changelog].
    pub async fn get_changelog(pool: &PgPool, cl_id: i64) -> Result<Option<Changelog>, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"SELECT * FROM changelog WHERE id = $1"#)
            .bind(cl_id)
            .fetch_optional(pool)
            .await
    }
    /// Check for if a given score already exists in the database, but is banned. Used for the auto-updating from Steam leaderboards.
    /// 
    /// Returns `true` if there is a value found, `false` if no value, or returns an error.
    pub async fn check_banned_scores(pool: &PgPool, params: ScoreLookup) -> std::result::Result<bool, sqlx::Error> {
        // We don't care about the result, we only care if there is a result.
        let res = sqlx::query(r#" 
                SELECT * 
                FROM changelog
                    INNER JOIN maps ON (maps.steam_id = changelog.map_id)
                    INNER JOIN chapters ON (chapters.id = maps.chapter_id)
                WHERE changelog.score = $1
                    AND changelog.map_id = $2
                    AND changelog.profile_number = $3
                    AND changelog.banned = $4
                    AND changelog.category_id = $5
                    AND chapters.game_id = $6"#)
            .bind(params.score)
            .bind(params.map_id)
            .bind(params.profile_number)
            .bind(true)
            .bind(params.cat_id.unwrap())
            .bind(params.game_id.unwrap())
            .fetch_optional(pool)
            .await?;
        match res {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    /// Returns a vec of [Changelog] for a user's personal best history on a given singleplayer map.
    /// 
    /// The function does not check to make sure that the map_id is singleplayer, but it is returned as a changelog entry,
    /// and no special join is done to handle coop-specific personal best history.
    pub async fn get_sp_pb_history(pool: &PgPool, profile_number: &str, map_id: &str, cat_id: i32, game_id: i32) -> Result<Vec<Changelog>, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#" 
                SELECT changelog.* 
                FROM changelog
                    INNER JOIN maps ON (maps.steam_id = changelog.map_id)
                    INNER JOIN chapters ON (chapters.id = maps.chapter_id)
                WHERE changelog.profile_number = $1
                    AND changelog.map_id = $2
                    AND changelog.category_id = $3
                    AND chapters.game_id = $4
                ORDER BY changelog.timestamp DESC NULLS LAST"#)
            .bind(profile_number)
            .bind(map_id)
            .bind(cat_id)
            .bind(game_id)
            .fetch_all(pool)
            .await
    }
    /// Deletes all references to a `demo_id` in `changelog`.
    pub async fn delete_references_to_demo(pool: &PgPool, demo_id: i64) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar(r#"UPDATE changelog SET demo_id = NULL WHERE demo_id = $1 RETURNING id;"#)
            .bind(demo_id)
            .fetch_all(pool)
            .await
    }
    /// Deletes all references to a `coop_id` in `changelog`
    #[allow(dead_code)]
    pub async fn delete_references_to_coop_id(pool: &PgPool, coop_id: i64) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar(r#"UPDATE changelog SET coop_id NULL WHERE coop_id = $1 RETURNING id;"#)
            .bind(coop_id)
            .fetch_all(pool)
            .await
    }
    /// Insert a new changelog entry.
    pub async fn insert_changelog(pool: &PgPool, cl: ChangelogInsert) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(r#"
                INSERT INTO changelog 
                (timestamp, profile_number, score, map_id, demo_id, banned, 
                youtube_id, coop_id, post_rank, pre_rank, submission, note,
                category_id, score_delta, verified, admin_note) VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING id"#)
            .bind(cl.timestamp).bind(cl.profile_number).bind(cl.score).bind(cl.map_id) // TODO: There has GOT to be a better way to do this... https://crates.io/crates/sqlxinsert ?
            .bind(cl.demo_id).bind(cl.banned).bind(cl.youtube_id).bind(cl.coop_id).bind(cl.post_rank)
            .bind(cl.pre_rank).bind(cl.submission).bind(cl.note).bind(cl.category_id)
            .bind(cl.score_delta).bind(cl.verified).bind(cl.admin_note)
            .fetch_one(pool)
            .await
    }
    /// Updates all fields (except ID) for a given changelog entry. Returns the updated [Changelog].
    pub async fn update_changelog(pool: &PgPool, update: Changelog) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"UPDATE changelog 
                SET timestamp = $1, profile_number = $2, score = $3, map_id = $4, demo_id = $5, banned = $6, 
                youtube_id = $7, coop_id = $8, post_rank = $9, pre_rank = $10, submission = $11, note = $12,
                category_id = $13, score_delta = $14, verified = $15, admin_note = $16
                WHERE id = $17 RETURNING *"#)
            .bind(update.timestamp).bind(update.profile_number).bind(update.score).bind(update.map_id) 
            .bind(update.demo_id).bind(update.banned).bind(update.youtube_id).bind(update.coop_id)
            .bind(update.post_rank).bind(update.pre_rank).bind(update.submission).bind(update.note)
            .bind(update.category_id).bind(update.score_delta).bind(update.verified).bind(update.admin_note)
            .bind(update.id)
            .fetch_one(pool)
            .await
    }
    /// Updates `demo_id` in a given changelog entry, returns the new [Changelog].
    pub async fn update_demo_id_in_changelog(pool: &PgPool, cl_id: i64, demo_id: i64) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"UPDATE changelog 
                SET demo_id = $1 WHERE id = $2 RETURNING *;"#)
            .bind(demo_id)
            .bind(cl_id)
            .fetch_one(pool)
            .await
    }
    #[allow(dead_code)]
    /// Deletes a changelog entry on the give ID.
    pub async fn delete_changelog(pool: &PgPool, cl_id: i64) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"DELETE FROM changelog WHERE id = $1 RETURNING *"#)
            .bind(cl_id)
            .fetch_one(pool)
            .await
    }
    #[allow(dead_code)]
    /// WIP: Testing Transactions in SQLX.
    pub async fn transaction_insert_changelog(transaction: &mut Transaction<'_>, cl: ChangelogInsert) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"
            INSERT INTO changelog 
                (timestamp, profile_number, score, map_id, demo_id, banned, 
                youtube_id, coop_id, post_rank, pre_rank, submission, note,
                category_id, score_delta, verified, admin_note) VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *"#)
            .bind(cl.timestamp).bind(cl.profile_number).bind(cl.score).bind(cl.map_id) // TODO: There has GOT to be a better way to do this... https://crates.io/crates/sqlxinsert ?
            .bind(cl.demo_id).bind(cl.banned).bind(cl.youtube_id).bind(cl.coop_id).bind(cl.post_rank)
            .bind(cl.pre_rank).bind(cl.submission).bind(cl.note).bind(cl.category_id)
            .bind(cl.score_delta).bind(cl.verified).bind(cl.admin_note)
            .fetch_one(&mut *transaction)
            .await
    }
    #[allow(dead_code)]
    /// WIP: Testing Transactions in SQLX.
    pub async fn transaction_update_changelog(transaction: &mut Transaction<'_>, update: Changelog) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"UPDATE changelog 
                SET timestamp = $1, profile_number = $2, score = $3, map_id = $4, demo_id = $5, banned = $6, 
                youtube_id = $7, coop_id = $8, post_rank = $9, pre_rank = $10, submission = $11, note = $12,
                category_id = $13, score_delta = $14, verified = $15, admin_note = $16
                WHERE id = $17 RETURNING *"#)
            .bind(update.timestamp).bind(update.profile_number).bind(update.score).bind(update.map_id) 
            .bind(update.demo_id).bind(update.banned).bind(update.youtube_id).bind(update.coop_id)
            .bind(update.post_rank).bind(update.pre_rank).bind(update.submission).bind(update.note)
            .bind(update.category_id).bind(update.score_delta).bind(update.verified).bind(update.admin_note)
            .bind(update.id)
            .fetch_one(&mut *transaction)
            .await
    }
    #[allow(dead_code)]
    /// WIP: Testing Transactions in SQLX.
    pub async fn transaction_delete_changelog(transaction: &mut Transaction<'_>, cl_id: i64) -> Result<Changelog, sqlx::Error> {
        sqlx::query_as::<_, Changelog>(r#"DELETE FROM changelog WHERE id = $1 RETURNING *"#)
            .bind(cl_id)
            .fetch_one(&mut *transaction)
            .await
    }
    
}

impl ChangelogPage {
    /// Display page for the changelog
    ///
    /// Takes a list of parameters, returns a filtered list of changelog entries.
    ///
    /// Returns a [ChangelogPage], which contains information specifc for displaying on the web.
    pub async fn get_changelog_page(
        pool: &PgPool,
        params: ChangelogQueryParams,
    ) -> Result<Vec<ChangelogPage>, sqlx::Error> {        
        let query_string = build_filtered_changelog(pool, params, None).await?;
        let res = sqlx::query_as::<_, ChangelogPage>(&query_string)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}

impl Graph {
    /// Return all [Maps] on a given `game_id`.
    pub async fn get_graph_data(
        pool: &PgPool
    ) -> Result<Vec<Graph>, sqlx::Error> {
        let res = sqlx::query_as::<_, Graph>(
            r#"
            SELECT DATE(timestamp) AS date, COUNT(*) AS count
            FROM changelog
            GROUP BY DATE(timestamp)
            ORDER BY DATE(timestamp) 
            DESC
            "#
        )
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}

/// Build a query String based off a pre-defined string. You pass in a [crate::models::changelog::ChangelogQueryParams], and an optional vector of additional filers.
/// 
/// Each element of the vector of additional filters will be assigned the correct "WHERE" or "AND", as appropriate.
/// 
/// ## Exanple use
/// ```rust
/// use crate::controllers::changelog::build_filtered_changelog;
/// 
/// async fn test_adding_filters() {
///     let mut additional_filters: Vec<String> =
///         vec!["(cl.banned = 'true' OR cl.verified = 'false' OR u.banned = 'true')".to_string(),
///         "u.profile_number = '76561198135023038'".to_string()];
///     let query_string = build_filtered_changelog(pool, params, Some(&mut additional_filters)).await.unwrap();
/// }
/// ```
/// 
pub async fn build_filtered_changelog(pool: &PgPool, params: ChangelogQueryParams, additional_filters: Option<&mut Vec<String>>) -> Result<String, sqlx::Error> {
    let mut query_string: String = String::from(
        r#" 
        SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned,
            cl.youtube_id, cl.previous_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
            cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name AS map_name,
            COALESCE(u.board_name, u.steam_name) AS user_name, u.avatar,
            COALESCE(p1.board_name, p1.steam_name) AS blue_name,
            COALESCE(p2.board_name, p2.steam_name) AS orange_name,
            p1.avatar AS blue_avatar, p2.avatar AS orange_avatar
                FROM changelog AS cl
                    INNER JOIN users AS u ON (u.profile_number = cl.profile_number)
                    INNER JOIN maps AS map ON (map.steam_id = cl.map_id)
                    INNER JOIN chapters AS chapter on (map.chapter_id = chapter.id)
                    LEFT JOIN coop_bundled AS coop on (cl.coop_id = coop.id)
                    LEFT JOIN users AS p1 ON coop.p_id1 = p1.profile_number
                    LEFT JOIN users AS p2 ON coop.p_id2 = p2.profile_number
    "#,
    );
    let mut filters: Vec<String> = Vec::new();
    if let Some(coop) = params.coop {
        if !coop {
            filters.push("chapter.is_multiplayer = False\n".to_string());
        } else if let Some(sp) = params.sp {
            if !sp {
                filters.push("chapter.is_multiplayer = True\n".to_string());
            }
        }
    }
    if let Some(has_demo) = params.has_demo {
        if has_demo {
            filters.push("cl.demo_id IS NOT NULL\n".to_string());
        } else {
            filters.push("cl.demo_id IS NULL\n".to_string());
        }
    }
    if let Some(yt) = params.yt {
        if yt {
            filters.push("cl.youtube_id IS NOT NULL\n".to_string());
        } else {
            filters.push("cl.youtube_id IS NULL\n".to_string());
        }
    }
    if let Some(wr_gain) = params.wr_gain {
        if wr_gain {
            filters.push("cl.post_rank = 1\n".to_string());
        }
    }
    if let Some(chamber) = params.chamber {
        filters.push(format!("cl.map_id = '{}'\n", &chamber));
    }
    if let Some(profile_number) = params.profile_number {
        filters.push(format!("cl.profile_number = {}\n", &profile_number));
    } else if let Some(nick_name) = params.nick_name {
        let mut profile_numbers = Users::check_board_name(pool, &nick_name).await?;
        if !profile_numbers.is_empty()
        {
            if profile_numbers.len() == 1 {
                filters.push(format!(
                    "cl.profile_number = '{}'\n",
                    &profile_numbers[0].to_string()
                ));
            } else {
                let mut profile_str = format!(
                    "(cl.profile_number = '{}'\n",
                    &profile_numbers[0].to_string()
                );
                profile_numbers.remove(0);
                for num in profile_numbers.iter() {
                    profile_str.push_str(&format!(" OR cl.profile_number = '{}'\n", num));
                }
                profile_str.push(')');
                filters.push(profile_str);
            }
        } else {
            return Err(sqlx::Error::RowNotFound);
        }
    }
    if let Some(first) = params.first {
        filters.push(format!("cl.id > {}\n", &first));
    } else if let Some(last) = params.last {
        filters.push(format!("cl.id < {}\n", &last));
    }
    if let Some(additional_filters) = additional_filters {
        filters.append(additional_filters);
    }
    // Build the statement based off the elements we added to our vector (used to make sure only first statement is WHERE, and additional are OR)
    for (i, entry) in filters.iter().enumerate() {
        if i == 0 {
            query_string = format!("{} WHERE {}", query_string, entry);
        } else {
            query_string = format!("{} AND {}", query_string, entry);
        }
    }
    //TODO: Maybe allow for custom order params????
    query_string = format!("{} ORDER BY cl.timestamp DESC NULLS LAST\n", query_string);
    if let Some(limit) = params.limit {
        query_string = format!("{} LIMIT {}\n", query_string, limit);
    } else {
        // TODO: Update to use the correct
        query_string = format!("{} LIMIT 500\n", query_string);
    }
    Ok(query_string)
}

impl Default for ChangelogQueryParams {
    fn default() -> Self {
        ChangelogQueryParams {
            limit: Some(500),
            nick_name: None,
            profile_number: None,
            chamber: None,
            sp: Some(true),
            coop: Some(true),
            wr_gain: None,
            has_demo: None,
            yt: None,
            first: None,
            last: None,
        }
    }
}

// TODO: Handle Autosubmit
impl ChangelogInsert {
    /// Create a [crate::models::changelog::ChangelogInsert] from a [crate::models::changelog::SubmissionChangelog]
    pub async fn new_from_submission(
        params: SubmissionChangelog,
        details: CalcValues,
        cache: &HashMap<String, i32>,
    ) -> ChangelogInsert {
        ChangelogInsert {
            timestamp: match NaiveDateTime::parse_from_str(&params.timestamp, "%Y-%m-%d %H:%M:%S") {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            profile_number: params.profile_number.clone(),
            score: params.score,
            map_id: params.map_id.clone(),
            youtube_id: params.youtube_id,
            note: params.note,
            category_id: params.category_id.unwrap_or_else(|| cache[&params.map_id]),
            submission: 1,
            previous_id: details.previous_id,
            post_rank: details.post_rank,
            pre_rank: details.pre_rank,
            score_delta: details.score_delta,
            banned: false,
            verified: Some(false),
            ..Default::default()
        }
    }
}