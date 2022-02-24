#![allow(dead_code)]

use anyhow::{Result, bail};
use std::collections::HashMap;
use sqlx::postgres::PgRow;
use sqlx::{Row, PgPool};
//use log::{debug};
use crate::controllers::models::*;

// TODO: Create macro for different lookup templates

impl Maps {
    /// Takes in a bool, if true returns MP map_ids, if false, returns as SP map_ids
    pub async fn get_steam_ids(pool: &PgPool, is_mp: bool) -> Result<Vec<String>> {
        let res = sqlx::query(r#"
                SELECT maps.steam_id FROM "p2boards".maps
                    INNER JOIN "p2boards".chapters ON (maps.chapter_id = chapters.id)
                    WHERE chapters.is_multiplayer = $1
                "#)
            .bind(is_mp)
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
    /// Returns the map name for a given steam_id.
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>> {
        let res = sqlx::query(r#"SELECT maps.name FROM "p2boards".maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns the default category for a given map.
    pub async fn get_deafult_cat(pool: &PgPool, map_id: String) -> Result<Option<i32>> {
        let res = sqlx::query(r#"
                SELECT default_cat_id FROM "p2boards".maps
                WHERE steam_id = $1;"#)
            .bind(map_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Returns chapter information for a given map_id (steam_id)
    pub async fn get_chapter_from_map_id(pool: &PgPool, map_id: String) -> Result<Option<Chapters>> {
        let res = sqlx::query_as::<_, Chapters>(r#"
                SELECT chapters.id, chapters.chapter_name, chapters.is_multiplayer, chapters.game_id
                FROM "p2boards".Chapters
                INNER JOIN "p2boards".maps ON (chapters.id = maps.chapter_id)
                WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Searches for all chapter IDs that match a given search pattern.
    pub async fn get_steam_id_by_name(pool: &PgPool, map_name: String) -> Result<Option<Vec<String>>> {
        let query_map_name = format!("%{}%", &map_name);
        let res = sqlx::query(r#"SELECT steam_id FROM "p2boards".maps 
                WHERE LOWER(name) LIKE LOWER($1)"#)
            .bind(query_map_name) 
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns true if the map is publicly accessible on the Steam Leaderboards.
    pub async fn get_is_public_by_steam_id(pool: &PgPool, map_id: String) -> Result<Option<bool>> {
        let res = sqlx::query(r#"SELECT is_public FROM "p2boards".maps WHERE steam_id = $1;"#)
            .bind(map_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
}

impl Chapters {
    /// Returns the maps for a given chapter.
    pub async fn get_map_ids(pool: &PgPool, chapter_id: i32) -> Result<Option<Vec<String>>> {
        let res = sqlx::query(r#"SELECT maps.steam_id FROM p2boards.maps WHERE chapter_id=$1"#)
            .bind(chapter_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        Ok(Some(res)) //We're not going to handle error cases I'm tired
        // TODO: Do this better
    }
    /// Searches for all chapter IDs that match a given search pattern.
    pub async fn get_chapter_id_by_name(pool: &PgPool, chapter_name: String) -> Result<Option<Vec<i32>>> {
        let query_chapter_name = format!("%{}%", &chapter_name);
        let res = sqlx::query(r#"SELECT id FROM "p2boards".chapters 
                WHERE LOWER(chapter_name) LIKE LOWER($1)"#)
            .bind(query_chapter_name) 
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns a chapter's data by the ID given.
    pub async fn get_chapter_by_id(pool: &PgPool, chapter_id: i32) -> Result<Option<Chapters>> {
        let res = sqlx::query_as::<_, Chapters>(r#"SELECT * FROM "p2boards".chapters WHERE id=$1;"#)
            .bind(chapter_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns true if the map is multiplayer, false if the map is singleplayer
    pub async fn get_chapter_is_multiplayer(pool: &PgPool, chapter_id: i32) -> Result<Option<bool>> {
        let res = sqlx::query(r#"SELECT is_multiplayer FROM "p2boards".chapters WHERE id=$1"#)
            .bind(chapter_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    pub async fn get_chapter_game(pool: &PgPool, chapter_id: i32) -> Result<Option<Games>> {
        let res = sqlx::query_as::<_, Games>(r#"SELECT games.id, games.game_name 
                FROM "p2boards".games
                INNER JOIN "p2boards".chapters ON (games.id = chapters.game_id)
                WHERE chapters.id = $1"#)
            .bind(chapter_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
}

impl Users {
    /// Returns user information
    pub async fn get_user(pool: &PgPool, profile_number: String) -> Result<Option<Users>> {
        let res = sqlx::query_as::<_, Users>(r#"SELECT * FROM "p2boards".users WHERE profile_number = $1"#)
            .bind(profile_number)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Gets a user's avatar and user_name/board_name (favors board_name)
    pub async fn get_user_data(pool: &PgPool, profile_number: String) -> Result<Option<UsersPage>> {
        let res = sqlx::query_as::<_, UsersPage>(r#"
                SELECT            
                CASE 
                    WHEN users.board_name IS NULL
                        THEN users.steam_name
                    WHEN users.board_name IS NOT NULL
                        THEN users.board_name
                    END user_name, users.avatar
                FROM "p2boards".users
                WHERE users.profile_number = $1
                "#)
            .bind(profile_number)
            .fetch_one(pool)
            .await;
        match res{
            Ok(user_data) => Ok(Some(user_data)),
            Err(e) => {
                eprintln!("User not found get_user_data -> {}", e);
                // return Err(anyhow::Error::new(e).context("Error with user data."))
                Ok(None)
            }
        }
    }
    /// Pattern match on a given string to find similar names (supports board/steam names). 
    pub async fn check_board_name(pool: &PgPool, nick_name: String) -> Result<Option<Vec<String>>> {
        let query_nn = format!("%{}%", &nick_name);
        let res = sqlx::query(r#"
                SELECT users.profile_number FROM "p2boards".users
                WHERE 
                    CASE
                        WHEN users.board_name IS NULL
                            THEN LOWER(users.steam_name) LIKE LOWER($1)
                        WHEN users.board_name IS NOT NULL
                            THEN LOWER(users.board_name) LIKE LOWER($1)
                    END
                "#) 
            .bind(query_nn)
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        if res.is_empty() {
            return Ok(None)
        }
        Ok(Some(res))
    }
    /// Returns a list of all banned player's profile_numbers.
    pub async fn get_banned(pool: &PgPool) -> Result<Vec<String>> {
        let res = sqlx::query(r#"SELECT users.profile_number FROM "p2boards".users WHERE users.banned = True"#)
            .map(|row: PgRow|{row.get(0)})    
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
    /// Returns the boolean flag associated with the user in the boards, if Err, assumed User does not exist.
    pub async fn check_banned(pool: &PgPool, profile_number: String) -> Result<bool> {
        let res = sqlx::query(r#"SELECT users.banned FROM "p2boards".users WHERE users.profile_number = $1"#)
            .bind(profile_number)
            .map(|row: PgRow| {row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Returns the title associated with the user (CAN BE NONE)
    pub async fn get_title(pool: &PgPool, profile_number: String) -> Result<Option<String>> {
        // Result of query can be None, None is valid and should not return an error.
        let res: Option<String> = sqlx::query(r#"SELECT title FROM "p2boards".users WHERE users.profile_number = $1"#)
            .bind(profile_number)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    } 
    /// Returns the social media informatio associated with a given user's profile_number
    pub async fn get_socials(pool: &PgPool, profile_number: String) -> Result<Option<Socials>> {
        let res = sqlx::query_as::<_, Socials>(r#"
                SELECT twitch, youtube, discord_id 
                FROM "p2boards".users 
                WHERE profile_number = $1"#)
            .bind(profile_number)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns the admin information associated with the user.
    pub async fn get_admin_for_user(pool: &PgPool, profile_number: String) -> Result<Option<i32>> {
        let res = sqlx::query(r#"SELECT admin FROM "p2boards".users WHERE profile_number = $1"#)
            .bind(profile_number)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns UsersPage for all admins
    /// Usage:  admin_value = 0     -> Non-admin user
    ///         admin_value = 1     -> Standard admin
    ///         admin_value = 2     -> Shadow admin
    ///             (Has admin permissions, is not publically listed)
    ///             (Typically reserved for former admins, trusted players)
    ///         admin_value = 3     -> Developer admin
    ///             (Has admin permissions as an activen developer only)
    pub async fn get_all_admins(pool: &PgPool, admin_value: i32) -> Result<Option<Vec<UsersPage>>> {
        let res = sqlx::query_as::<_, UsersPage>(r#"
                SELECT            
                CASE 
                    WHEN users.board_name IS NULL
                        THEN users.steam_name
                    WHEN users.board_name IS NOT NULL
                        THEN users.board_name
                    END user_name, users.avatar
                FROM "p2boards".users
                WHERE users.admin = $1
                "#)
            .bind(admin_value)
            .fetch_all(pool)
            .await;
        match res {
            Ok(user_data) => Ok(Some(user_data)),
            Err(e) => {
                eprintln!("User not found get_user_data -> {}", e);
                // return Err(anyhow::Error::new(e).context("Error with user data."))
                Ok(None)
            }
        }
    }
    // TODO: Consider using profanity filter (only for really bad names): https://docs.rs/censor/latest/censor/
    /// Inserts a new user into the databse
    pub async fn insert_new_users(pool: &PgPool, new_user: Users) -> Result<bool> {
        let mut res = String::new();
        // We do not care about the returning profile_number. As it is not generated and we already have it
        let _ = sqlx::query(r#"
                INSERT INTO "p2boards".Users
                (profile_number, board_name, steam_name, banned, registered, 
                avatar, twitch, youtube, title, admin, donation_amount, discord_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING profile_number"#)
            .bind(new_user.profile_number.clone()).bind(new_user.board_name).bind(new_user.steam_name)
            .bind(new_user.banned).bind(new_user.registered).bind(new_user.avatar)
            .bind(new_user.twitch).bind(new_user.youtube).bind(new_user.title)
            .bind(new_user.admin).bind(new_user.donation_amount).bind(new_user.discord_id)
            .map(|row: PgRow| {
                res = row.get(0);
            })
            .fetch_one(pool)
            .await?;
        if res == new_user.profile_number {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub async fn update_existing_user(pool: &PgPool, updated_user: Users) -> Result<bool> {
        // If this gives us an error, we're updaing a user that already exists.
        let _ = Users::get_user(pool, updated_user.profile_number.clone()).await?;
        // TODO: Check to make sure user has correct AUTH to update specific items
        // (board_name should only be changed by the backend, admin should only be updated by admin etc)
        let _ = sqlx::query(r#"
                UPDATE "p2boards".Users
                SET board_name = $1, steam_name = $2, banned = $3, registered = $4, 
                avatar = $5, twitch = $6, youtube = $7, title = $8, admin = $9,
                donation_amount = $10, discord_id = $11
                WHERE profile_number = $12"#)
            .bind(updated_user.board_name).bind(updated_user.steam_name)
            .bind(updated_user.banned).bind(updated_user.registered).bind(updated_user.avatar)
            .bind(updated_user.twitch).bind(updated_user.youtube).bind(updated_user.title)
            .bind(updated_user.admin).bind(updated_user.donation_amount)
            .bind(updated_user.discord_id).bind(updated_user.profile_number)
            .fetch_optional(pool)
            .await?;
        Ok(true)
    }
    pub async fn delete_user(pool: &PgPool, profile_number: String) -> Result<bool> {
        let res = sqlx::query_as::<_, Users>(r#"DELETE FROM "p2boards".users 
                WHERE profile_number = $1 RETURNING *"#)
            .bind(profile_number)
            .fetch_one(pool)
            .await;
        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Error deleting user -> {}", e);
                Ok(false)
            },
        }
    }
}

impl Demos {
    /// Gets Demo information for a given demo_id
    pub async fn get_demo(pool: &PgPool, demo_id: i64) -> Result<Option<Demos>> {
        let res = sqlx::query_as::<_, Demos>(r#"SELECT * FROM "p2boards".demos WHERE id = $1"#)
            .bind(demo_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns a file id that can be used to download the demo
    pub async fn get_demo_file_id(pool: &PgPool, demo_id: i64) -> Result<Option<String>> {
        let res = sqlx::query(r#"SELECT file_id FROM "p2boards".demos WHERE id = $1"#)
            .bind(demo_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns the partner's name
    pub async fn get_partner_name(pool: &PgPool, demo_id: i64) -> Result<Option<String>> {
        let res = sqlx::query(r#"SELECT partner_name FROM "p2boards".demos WHERE id = $1"#)
            .bind(demo_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Check to see if a demo was parsed successfully
    pub async fn check_parsed(pool: &PgPool, demo_id: i64) -> Result<bool> {
        let res = sqlx::query(r#"SELECT parsed_successfully FROM "p2boards".demos WHERE id = $1"#)
            .bind(demo_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Gets the SAR version associated with a demo
    pub async fn get_sar_version(pool: &PgPool, demo_id: i64) -> Result<Option<String>> {
        let res = sqlx::query(r#"SELECT sar_version FROM "p2boards".demos WHERE id = $1"#)
            .bind(demo_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Adds a new demo to the database, returns the demo's id
    pub async fn insert_demo(pool: &PgPool, demo: DemoInsert) -> Result<i64> {
        let mut res: i64 = 0; 
        let _ = sqlx::query(r#"
                INSERT INTO "p2boards".demos 
               
                (file_id, partner_name, parsed_successfully, sar_version, cl_id) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#)
            .bind(demo.file_id).bind(demo.partner_name).bind(demo.parsed_successfully).bind(demo.sar_version).bind(demo.cl_id)
            .map(|row: PgRow|{res = row.get(0)})
            .fetch_one(pool)
            .await?;
            Ok(res)
    }
    /// Updates an existing demo
    pub async fn update_demo(pool: &PgPool, updated_demo: Demos) -> Result<bool> {
        // TODO: Validation
        let _ = sqlx::query(r#"
                UPDATE "p2boards".demos
                SET file_id = $1, partner_name = $2, parsed_successfully = $3,
                sar_version = $4, cl_id = $5
                WHERE id = $6"#)
            .bind(updated_demo.file_id).bind(updated_demo.partner_name)
            .bind(updated_demo.parsed_successfully).bind(updated_demo.sar_version)
            .bind(updated_demo.cl_id).bind(updated_demo.id)
            .fetch_optional(pool)
            .await?;
        Ok(true)
    }
    /// Deletes a demo
    pub async fn delete_demo(pool: &PgPool, demo_id: i64) -> Result<bool> {
        let res = sqlx::query_as::<_, Demos>(r#"DELETE FROM "p2boards".demos 
                WHERE id = $1 RETURNING *"#)
            .bind(demo_id)
            .fetch_one(pool)
            .await;
        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Error deleting demo -> {}", e);
                Ok(false)
            },
        }        
    }
}

// Implementations of associated functions for Changelog
impl Changelog {
    pub async fn get_changelog(pool: &PgPool, cl_id: i64) -> Result<Option<Changelog>> {
        let res = sqlx::query_as::<_, Changelog>(r#"SELECT * FROM "p2boards".changelog WHERE id = $1"#)
            .bind(cl_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    /// Check for if a given score already exists in the database, but is banned. Used for the auto-updating from Steam leaderboards.
    /// Returns `true` if there is a value found, `false` if no value, or returns an error.
    pub async fn check_banned_scores(pool: &PgPool, map_id: String, score: i32, profile_number: String) -> Result<bool> {
        // We don't care about the result, we only care if there is a result.
        let res = sqlx::query(r#" 
                SELECT * 
                FROM "p2boards".changelog
                WHERE changelog.score = $1
                AND changelog.map_id = $2
                AND changelog.profile_number = $3
                AND changelog.banned = $4"#)
            .bind(score)
            .bind(map_id)
            .bind(profile_number)
            .bind(true)
            .fetch_optional(pool)
            .await?;
        match res {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    // Returns a vec of changelog for a user's PB history on a given SP map.
    pub async fn get_sp_pb_history(pool: &PgPool, profile_number: String, map_id: String) -> Result<Vec<Changelog>> {
        let res = sqlx::query_as::<_, Changelog>(r#" 
                SELECT * 
                FROM "p2boards".changelog
                WHERE changelog.profile_number = $1
                AND changelog.map_id = $2
                ORDER BY changelog.timestamp DESC NULLS LAST"#)
            .bind(profile_number)
            .bind(map_id)
            .fetch_all(pool)
            .await;
        match res{
            Ok(pb_history) => Ok(pb_history),
            Err(e) => Err(anyhow::Error::new(e).context("Could not find SP PB History")),
        }
    }
    /// Insert a new changelog entry.
    pub async fn insert_changelog(pool: &PgPool, cl: ChangelogInsert) -> Result<i64> {
        // TODO: https://stackoverflow.com/questions/4448340/postgresql-duplicate-key-violates-unique-constraint
        let mut res: i64 = 0; 
        let _ = sqlx::query(r#"
                INSERT INTO "p2boards".changelog 
                (timestamp, profile_number, score, map_id, demo_id, banned, 
                youtube_id, coop_id, post_rank, pre_rank, submission, note,
                category_id, score_delta, verified, admin_note) VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING id"#)
            .bind(cl.timestamp).bind(cl.profile_number).bind(cl.score).bind(cl.map_id) // TODO: There has GOT to be a better way to do this... https://crates.io/crates/sqlxinsert ?
            .bind(cl.demo_id).bind(cl.banned).bind(cl.youtube_id).bind(cl.coop_id).bind(cl.post_rank)
            .bind(cl.pre_rank).bind(cl.submission).bind(cl.note).bind(cl.category_id)
            .bind(cl.score_delta).bind(cl.verified).bind(cl.admin_note)
            .map(|row: PgRow|{res = row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Updates all fields (except ID) for a given changelog entry. Returns the updated Changelog struct.
    pub async fn update_changelog(pool: &PgPool, update: Changelog) -> Result<bool> {
        let _ = sqlx::query(r#"UPDATE "p2boards".changelog 
                SET timestamp = $1, profile_number = $2, score = $3, map_id = $4, demo_id = $5, banned = $6, 
                youtube_id = $7, coop_id = $8, post_rank = $9, pre_rank = $10, submission = $11, note = $12,
                category_id = $13, score_delta = $14, verified = $15, admin_note = $16
                WHERE id = $17"#)
            .bind(update.timestamp).bind(update.profile_number).bind(update.score).bind(update.map_id) 
            .bind(update.demo_id).bind(update.banned).bind(update.youtube_id).bind(update.coop_id)
            .bind(update.post_rank).bind(update.pre_rank).bind(update.submission).bind(update.note)
            .bind(update.category_id).bind(update.score_delta).bind(update.verified).bind(update.admin_note)
            .bind(update.id)
            .fetch_optional(pool)
            .await?;
        Ok(true)
    }
    pub async fn delete_changelog(pool: &PgPool, cl_id: i64) -> Result<bool> {
        let res = sqlx::query_as::<_, Changelog>(r#"DELETE FROM "p2boards".changelog WHERE id = $1 RETURNING *"#)
            .bind(cl_id)
            .fetch_one(pool)
            .await;
        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Error deleting demo -> {}", e);
                Ok(false)
            },
        }
    }  
}

impl CoopBundled {
    pub async fn insert_coop_bundled(pool: &PgPool, cl: CoopBundledInsert) -> Result<i64> {
        let mut res: i64 = 0;
        let _ = sqlx::query(r#"
                INSERT INTO "p2boards".coop_bundled 
                (p_id1, p_id2, p1_is_host, cl_id1, cl_id2) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#)
            .bind(cl.p_id1).bind(cl.p_id2).bind(cl.p1_is_host).bind(cl.cl_id1).bind(cl.cl_id2)
            .map(|row: PgRow|{res = row.get(0)})
            .fetch_one(pool)
            .await?;
            Ok(res)
    }
}


impl ChangelogPage {
    // Gets as many changelog entries as the limit passed.
    // TODO: Base this on time rather than a hard limit??
    pub async fn get_cl_page(pool: &PgPool, limit: i32) -> Result<Option<Vec<ChangelogPage>>> {
        let res = sqlx::query_as::<_, ChangelogPage>(r#" 
                SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned, 
                cl.youtube_id, cl.previous_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
                cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name AS map_name, 
                CASE
                    WHEN u.board_name IS NULL
                        THEN u.steam_name
                    WHEN u.board_name IS NOT NULL
                        THEN u.board_name
                END user_name, u.avatar
                FROM "p2boards".changelog AS cl
                INNER JOIN "p2boards".users AS u ON (u.profile_number = cl.profile_number)
                INNER JOIN "p2boards".maps AS map ON (map.steam_id = cl.map_id)
                ORDER BY cl.timestamp DESC NULLS LAST
                LIMIT $1"#)
            .bind(limit)
            .fetch_all(pool)
            .await;
        match res{
            Ok(res) => Ok(Some(res)),
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
    }
    // Handles filtering out changelog by different criteria.
    pub async fn get_cl_page_filtered(pool: &PgPool, params: ChangelogQueryParams) -> Result<Option<Vec<ChangelogPage>>> {
        // TODO: Add additonal filters
        let mut filters: Vec<String> = Vec::new();
        let mut query_string: String = String::from(r#" 
            SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned, 
            cl.youtube_id, cl.previous_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
            cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name AS map_name,  
            CASE
                WHEN u.board_name IS NULL
                    THEN u.steam_name
                WHEN u.board_name IS NOT NULL
                    THEN u.board_name
            END user_name, u.avatar
            FROM "p2boards".changelog AS cl
            INNER JOIN "p2boards".users AS u ON (u.profile_number = cl.profile_number)
            INNER JOIN "p2boards".maps AS map ON (map.steam_id = cl.map_id)
            INNER JOIN "p2boards".chapters AS chapter on (map.chapter_id = chapter.id)
        "#);
        if let Some(coop) = params.coop {
            if !coop {
                filters.push("chapters.is_multiplayer = False\n".to_string());
            } else if let Some(sp) = params.sp {
                if !sp {
                    filters.push("chapters.is_multiplayer = True\n".to_string());
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
        }
        //#[allow(irrefutable_let_patterns)]
        if let Some(nick_name) = params.nick_name {
            //eprintln!("{}", nick_name);
            if let Some(profile_numbers) = Users::check_board_name(pool, nick_name.clone()).await?.as_mut(){
                if profile_numbers.len() == 1 {
                    filters.push(format!("cl.profile_number = '{}'\n", &profile_numbers[0].to_string()));
                } else {
                    let mut profile_str = format!("(cl.profile_number = '{}'\n", &profile_numbers[0].to_string());
                    profile_numbers.remove(0);
                    for num in profile_numbers.iter(){
                        profile_str.push_str(&format!(" OR cl.profile_number = '{}'\n", num));
                    }
                    profile_str.push_str(")");
                    filters.push(profile_str);
                }
            }
            else {
                bail!("No users found with specified username pattern.");
            }
        }
        // Build the statement based off the elements we added to our vector (used to make sure only first statement is WHERE, and additional are OR)
        for (i, entry) in filters.iter().enumerate() {
            if i == 0 {
                query_string = format!("{} WHERE {}", &query_string, entry);
            } else {
                query_string = format!("{} AND {}", &query_string, entry);
            }
        }
        //TODO: Maybe allow for custom order params????
        query_string = format!("{} ORDER BY cl.timestamp DESC NULLS LAST\n", &query_string);
        if let Some(limit) = params.limit{
            query_string = format!("{} LIMIT {}\n", &query_string, limit);
        } else{ // Default limit
            query_string = format!("{} LMIT 1000\n", &query_string)
        }
        // eprintln!("{}", query_string);

        let res = sqlx::query_as::<_, ChangelogPage>(&query_string).fetch_all(pool).await;
        match res{
            Ok(changelog_filtered) => Ok(Some(changelog_filtered)),
            Err(e) => {
                eprintln!("{}", query_string);
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
        // Ok(Some(res))
    }
}

impl SpMap {
    pub async fn get_sp_map_page(pool: &PgPool, map_id: String, limit: i32, cat_id: Option<i32>) -> Result<Vec<SpMap>> {
        let category_id: i32;
        if let Some(x) = cat_id {
            category_id = x;
        } else {
            let dcid = Maps::get_deafult_cat(&pool, map_id.clone()).await;
            category_id = match dcid {
                Ok(Some(id)) => id,
                _ => bail!("Could not find a default cat_id for the map provided"),
            };
        }
        let res = sqlx::query_as::<_, SpMap>(r#" 
                SELECT t.timestamp,
                    t.CL_profile_number,
                    t.score,
                    t.demo_id,
                    t.youtube_id,
                    t.submission,
                    t.note,
                    t.category_id,
                    CASE
                    WHEN t.board_name IS NULL
                        THEN t.steam_name
                    WHEN t.board_name IS NOT NULL
                        THEN t.board_name
                    END user_name,
                    t.avatar
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM "p2boards".changelog
                    INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
                        WHERE map_id = $1
                        AND users.banned = False
                        AND changelog.verified = True
                        AND changelog.banned = False
                        AND changelog.category_id = $2
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score
                LIMIT $3"#)
            .bind(map_id)
            .bind(category_id)
            .bind(limit)
            .fetch_all(pool)
            .await;
        match res{
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
        //Ok(res)
    }
}

impl CoopMap {
    // TODO: Check to make sure this is filtered when returned (I think it is).
    pub async fn get_coop_map_page(pool: &PgPool, map_id: String, limit: i32, cat_id: Option<i32>) -> Result<Vec<CoopMap>> {
        let category_id: i32;
        if let Some(x) = cat_id {
            category_id = x;
        } else {
            let dcid = Maps::get_deafult_cat(&pool, map_id.clone()).await;
            category_id = match dcid {
                Ok(Some(id)) => id,
                _ => bail!("Could not find a default cat_id for the map provided"),
            };
        }
        let res = sqlx::query_as::<_, CoopMap>(r#"
                SELECT  c1.timestamp, 
                    c1.score, cb.p1_is_host, c1.note AS note1, c2.note AS note2,
                    CASE 
                        WHEN p1.board_name IS NULL
                            THEN p1.steam_name
                        WHEN p1.board_name IS NOT NULL
                            THEN p1.board_name
                    END user_name1, 
                        CASE 
                        WHEN p2.board_name IS NULL
                            THEN p2.steam_name
                        WHEN p2.board_name IS NOT NULL
                            THEN p2.board_name
                    END user_name2,
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2, 
                    c1.demo_id AS demo_id1, c2.demo_id AS demo_id2, 
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2,
                    c1.submission AS submission1, c2.submission AS submission2, 
                    c1.category_id, p1.avatar AS avatar1, p2.avatar AS avatar2
                FROM (SELECT * FROM 
                "p2boards".coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
                WHERE p1.banned=False
                    AND p2.banned=False
                    AND c1.banned=False
                    AND c2.banned=False
                    AND c1.verified=True
                    AND c2.verified=True
                    AND c1.category_id=$2
                ORDER BY score ASC
                "#)
            .bind(map_id)
            .bind(category_id)
            .fetch_all(pool)
            .await;
        match res {
            Ok(mut res) => {
                res.truncate(limit as usize);
                Ok(res)
            },
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
    }
}

impl SpPreview {
    /// Gets preview information for top 7 on an SP Map.
    pub async fn get_sp_preview(pool: &PgPool, map_id: String) -> Result<Vec<SpPreview>> {
        // use std::time::Instant;
        // let now = Instant::now();
        let res = sqlx::query_as::<_, SpPreview>(r#"
                SELECT t.CL_profile_number, t.score, t.youtube_id, t.category_id,
                CASE
                    WHEN t.board_name IS NULL
                        THEN t.steam_name
                    WHEN t.board_name IS NOT NULL
                        THEN t.board_name
                    END user_name, t.map_id
                FROM (
                    SELECT DISTINCT ON (changelog.profile_number) 
                        changelog.profile_number as CL_profile_number,
                        users.profile_number as U_profile_number, *
                    FROM "p2boards".changelog
                    INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
                    WHERE map_id = $1
                    AND users.banned = False
                    AND changelog.banned = False
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
               ORDER BY score
               LIMIT 7"#)
            .bind(map_id.clone())
            .fetch_all(pool)
            .await;
        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);
        match res{
            Ok(sp_previews) => Ok(sp_previews),
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Previews"))
            }
        }
    }
}

impl SpPreviews{
    /// Collects the top 7 preview data for all SP maps.
    pub async fn get_sp_previews(pool: &PgPool) -> Result<Vec<SpPreviews>> {
        let map_id_vec = Maps::get_steam_ids(pool, false).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter(){
            let vec_temp = SpPreview::get_sp_preview(pool, map_id.to_string()).await?;
            vec_final.push(SpPreviews{
                map_id : map_id.clone(),
                scores : vec_temp,
            })
        }
        Ok(vec_final)
    }
}

impl CoopPreview {
    /// Gets the top 7 (unique on player) times on a given Coop Map.
    pub async fn get_coop_preview(pool: &PgPool, map_id: String) -> Result<Vec<CoopPreview>> {
        // TODO: Open to PRs to contain all this functionality in the SQL statement.
        let query = sqlx::query_as::<_, CoopPreview>(r#"
                SELECT
                    c1.profile_number AS profile_number1, c2.profile_number AS profile_number2,
                    c1.score,
                    c1.youtube_id AS youtube_id1, c2.youtube_id AS youtube_id2, c1.category_id,
                    CASE 
                    WHEN p1.board_name IS NULL
                        THEN p1.steam_name
                    WHEN p1.board_name IS NOT NULL
                        THEN p1.board_name
                    END user_name1, 
                    CASE 
                    WHEN p2.board_name IS NULL
                        THEN p2.steam_name
                    WHEN p2.board_name IS NOT NULL
                        THEN p2.board_name
                    END user_name2
                FROM (SELECT * FROM 
                "p2boards".coop_bundled 
                WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = '47825'
                    AND coop_id IS NOT NULL)) as cb 
                INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
                WHERE p1.banned=False
                    AND p2.banned=False
                    AND c1.banned=False
                    AND c2.banned=False
                    AND c1.verified=True
                    AND c2.verified=True
                ORDER BY score ASC
                LIMIT 40
                "#)
            .bind(map_id.clone())
            .fetch_all(pool)
            .await;
        match query{
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Previews"))
            }
        }
        // TODO: Maybe remove unwrap(), it assumes that the profile_number2 will not be None.
        let mut vec_final = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(80);
        remove_dups.insert("N/A".to_string(), 1);
        for entry in query.unwrap(){
            match remove_dups.insert(entry.profile_number1.clone(), 1){
                Some(_) => match remove_dups.insert(entry.profile_number2.clone().unwrap(), 1){
                    Some(_) => (),
                    _ => vec_final.push(entry),
                }
                _ => match remove_dups.insert(entry.profile_number2.clone().unwrap(), 1){
                    Some(_) => vec_final.push(entry),
                    _ => vec_final.push(entry),
                }    
            }
        }
        vec_final.truncate(7);
        Ok(vec_final)
    }
}

impl CoopPreviews {
    // Collects the top 7 preview data for all Coop maps.
    pub async fn get_coop_previews(pool: &PgPool) -> Result<Vec<CoopPreviews>> {
        let map_id_vec = Maps::get_steam_ids(pool, true).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter(){
            let vec_temp = CoopPreview::get_coop_preview(pool, map_id.to_string()).await?;
            vec_final.push(CoopPreviews{
                map_id : map_id.clone(),
                scores : vec_temp,
            })
        }
        Ok(vec_final)      
    }
}

impl SpBanned {
    // Returns all profile_numbers and scores associated with banned times on a given map
    pub async fn get_sp_banned(pool: &PgPool, map_id: String) -> Result<Vec<SpBanned>> {
        let res = sqlx::query_as::<_, SpBanned>(r#"
                SELECT changelog.profile_number, changelog.score 
                    FROM "p2boards".changelog
                    WHERE changelog.banned = True
                        AND changelog.map_id = $1
                    ORDER BY changelog.score ASC
            "#)
            .bind(map_id)
            .fetch_all(pool)
            .await;   
        match res{
            Ok(sp_banned) => Ok(sp_banned),
            Err(e) => {
                eprintln!("{}", e);
                Err(anyhow::Error::new(e).context("Error with SP Banned."))
            }
        }
    }
}

impl CoopBanned {
    /// Currently returns two profile_numbers and a score associated with a coop_bundle where one or both times are either banned or unverifed.
    pub async fn get_coop_banned(pool: &PgPool, map_id: String) -> Result<Vec<CoopBanned>> {
        // TODO: Handle verified and handle if one is banned/not verified but the other isn't.
        // TODO: How to handle one player in coop not-being banned/unverified but the other is.
        let res = sqlx::query_as::<_, CoopBanned>(r#"
                SELECT c1.score, p1.profile_number AS profile_number1, p2.profile_number AS profile_number2
                FROM (SELECT * FROM 
                    "p2boards".coop_bundled 
                    WHERE id IN 
                    (SELECT coop_id
                    FROM "p2boards".changelog
                    WHERE map_id = $1
                    AND coop_id IS NOT NULL)) as cb
                INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
                INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
                INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
                INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
                    WHERE (c1.banned = True OR c1.verified = False)
                    OR (c2.banned = True OR c2.verified = False)
                "#)
            .bind(map_id)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}