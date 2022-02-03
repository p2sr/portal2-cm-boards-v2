#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error, HttpRequest, Responder};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row, PgPool};
use futures::future::{ready, Ready};
use anyhow::Result;
use chrono::NaiveDateTime;
use log::{debug};


use crate::tools::datamodels::*;

impl CoopPreview {
    /// Gets the top 7 (unique on player) times on a given Coop Map.
    pub async fn get_coop_preview(pool: &PgPool, map_id: String) -> Result<Vec<CoopPreview>>{
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
    pub async fn get_coop_previews(pool: &PgPool) -> Result<Vec<CoopPreviews>>{
        let map_id_vec = Maps::get_steamids(&pool, true).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter(){
            let vec_temp = CoopPreview::get_coop_preview(&pool, map_id.to_string()).await?;
            vec_final.push(CoopPreviews{
                map_id : map_id.clone(),
                scores : vec_temp,
            })
        }
        Ok(vec_final)      
    }
}

impl Chapters{
    pub async fn get_map_ids(pool: &PgPool, chapter_id: i32) -> Result<Option<Vec<String>>>{
        let res = sqlx::query(r#"SELECT maps.steam_id FROM p2boards.maps WHERE chapter_id=$1"#)
            .bind(chapter_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_all(pool)
            .await?;
        Ok(Some(res)) //We're not going to handle error cases I'm tired
        // TODO: Do this better
    }
}

impl SpPreview{
    /// Gets preview information for top 7 on an SP Map.
    pub async fn get_sp_preview(pool: &PgPool, map_id: String) -> Result<Vec<SpPreview>>{
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
        match res{
            Ok(sp_previews) => Ok(sp_previews),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Previews"))
            }
        }
    }
}

impl SpPreviews{
    // Collects the top 7 preview data for all SP maps.
    pub async fn get_sp_previews(pool: &PgPool) -> Result<Vec<SpPreviews>>{
        let map_id_vec = Maps::get_steamids(&pool, false).await?;
        let mut vec_final = Vec::new();
        for map_id in map_id_vec.iter(){
            let vec_temp = SpPreview::get_sp_preview(&pool, map_id.to_string()).await?;
            vec_final.push(SpPreviews{
                map_id : map_id.clone(),
                scores : vec_temp,
            })
        }
        Ok(vec_final)
    }
}

impl SpBanned{
    // Returns all profile_numbers and scores associated with banned times on a given map
    pub async fn get_sp_banned(pool: &PgPool, map_id: String) -> Result<Vec<SpBanned>>{
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
                return Err(anyhow::Error::new(e).context("Error with SP Banned."))
            }
        }
    }
}

impl CoopBanned{
    pub async fn get_coop_banned(pool: &PgPool, map_id: String) -> Result<Vec<CoopBanned>>{
        // TODO: Handle verified and handle if one is banned/not verified but the other isn't.
        // TODO: How to handle one player in coop not-being banned/unverified but the other is.
        /// Currently returns two profile_numbers and a score associated with a coop_bundle where one or both times are either banned or unverifed.
        let res = sqlx::query_as::<_, CoopBanned>(r#"
                SELECT c1.score, p1.profile_number, p2.profile_number
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

impl Maps{
    /// Takes in a bool, if true returns ass MP map_ids, if false, returns as SP map_ids
    pub async fn get_steamids(pool: &PgPool, is_mp: bool) -> Result<Vec<String>>{
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
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>>{
        let res = sqlx::query(r#"SELECT maps.name FROM "p2boards".maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
    pub async fn get_deafult_cat(pool: &PgPool, map_id: String) -> Result<Option<i32>> {
        // TODO: This "deafult" is kind of stupid. There are 108 defaults so we just make sure the ID is < 109
        let res = sqlx::query(r#"
                SELECT ct.id FROM "p2boards".categories
                    AS ct INNER JOIN "p2boards".maps AS map ON (ct.map_id = map.steam_id)
                    WHERE (ct.id < 109) AND (map.steam_id = $1);"#)
            .bind(map_id)
            .map(|row: PgRow|{row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
}

impl Users{
    pub async fn get_user_data(pool: &PgPool, profile_number: String) -> Result<Option<UsersPage>>{
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
                eprintln!("User not found get_user_data ->{}", e);
                // return Err(anyhow::Error::new(e).context("Error with user data."))
                return Ok(None)
            }
        }
    }
    /// Pattern match on a given string to find similar names (supports board/steam names). 
    pub async fn check_board_name(pool: &PgPool, nick_name: String) -> Result<Option<Vec<String>>>{ // TODO: Check return type of 0 results more carefully
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
        Ok(Some(res))
    }
    /// Returns a list of all banned players profile_numbers.
    pub async fn get_banned(pool: &PgPool) -> Result<Vec<String>>{
        let res = sqlx::query(r#"SELECT users.profile_number FROM "p2boards".users WHERE users.banned = True"#)
            .map(|row: PgRow|{row.get(0)})    
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
    /// Returns a boolean based on if the profile number passed is banned or not.
    pub async fn check_banned(pool: &PgPool, profile_number: String) -> Result<bool>{
        let res = sqlx::query(r#"SELECT users.banned FROM "p2boards".users WHERE users.profile_number = $1"#)
            .bind(profile_number)
            .map(|row: PgRow| {row.get(0)})
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
    /// Inserts a new user into the databse
    pub async fn insert_new_users(pool: &PgPool, new_user: Users) -> Result<bool> {
        let mut res = String::new();
        let query = sqlx::query(r#"
                INSERT INTO "p2boards".Users
                (profile_number, board_name, steam_name, banned, registred, 
                avatar, twitch, youtube, title, admin, donation_amount, discord_id)
                VALUSE ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING profile_number"#)
            .bind(new_user.profile_number.clone()).bind(new_user.board_name).bind(new_user.steam_name)
            .bind(new_user.banned).bind(new_user.registred).bind(new_user.avatar)
            .bind(new_user.twitch).bind(new_user.youtube).bind(new_user.title)
            .bind(new_user.admin).bind(new_user.donation_amount).bind(new_user.discord_id)
            .map(|row: PgRow| {
                res = row.get(0);
            })
            .fetch_one(pool)
            .await?;
        if res == new_user.profile_number {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

impl CoopMap{
    // TODO: Check to make sure this is filtered when returned (I think it is).
    pub async fn get_coop_map_page(pool: &PgPool, map_id: String) -> Result<Vec<CoopMap>> {
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
                ORDER BY score ASC
                "#)
            .bind(map_id)
            .fetch_all(pool)
            .await;
        match res{
            Ok(res) => return Ok(res),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
    }
}
impl SpMap{
    pub async fn get_sp_map_page(pool: &PgPool, map_id: String) -> Result<Vec<SpMap>> {
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
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score
                LIMIT 200"#)
            .bind(map_id)
            .fetch_all(pool)
            .await;
        match res{
            Ok(res) => return Ok(res),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
        //Ok(res)
    }
}

impl CoopBundled {
    pub async fn insert_coop_bundled(pool: &PgPool, cl: CoopBundledInsert) -> Result<i64>{
        let mut res: i64 = 0; 
        let query = sqlx::query(r#"
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

// Implementations of associated functions for Changelog
impl Changelog{
    /// Check for if a given score already exists in the database, but is banned. Used for the auto-updating from Steam leaderboards.
    /// Returns `true` if there is a value found, `false` if no value, or returns an error.
    pub async fn check_banned_scores(pool: &PgPool, map_id: String, score: i32, profile_number: String) -> Result<bool>{
        // We don't care about the result, we only care if there is a result.
        let query = sqlx::query(r#" 
                SELECT * 
                FROM "p2boards".changelog
                WHERE changelog.score = $1
                AND changelog.map_id = $2
                AND changelog.profile_number = $3
                AND changelog.banned = $4"#)
            .bind(score)
            .bind(map_id)
            .bind(profile_number)
            .bind(true);
        let res = query.fetch_optional(pool).await?;
        match res{
            Some(_) => return Ok(true),
            None => return Ok(false),
        }
    }
    // Returns a vec of changelog for a user's PB history on a given SP map.
    pub async fn get_sp_pb_history(pool: &PgPool, profile_number: String, map_id: String) -> Result<Vec<Changelog>>{
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
            Ok(pb_history) => return Ok(pb_history),
            Err(e) => {
                eprintln!("Error with get_sp_pb_history {}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
    }
    /// Insert a new changelog entry.
    pub async fn insert_changelog(pool: &PgPool, cl: ChangelogInsert) -> Result<i64> {
        // TODO: https://stackoverflow.com/questions/4448340/postgresql-duplicate-key-violates-unique-constraint
        let mut res: i64 = 0; 
        let query = sqlx::query(r#"
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
    pub async fn update_changelog(pool: &PgPool, update: Changelog) -> Result<Changelog>{
        let res = sqlx::query_as::<_, Changelog>(r#"
                UPDATE "p2boards".changelog
                SET timestamp = $1, profile_number = $2, score = $3, map_id = $4, demo_id = $5, banned = $6, 
                youtube_id = $7, coop_id = $8, post_rank = $9, pre_rank = $10, submission = $11, note = $12,
                category_id = $13, score_delta = $14, verified = $15, admin_note = $15
                WHERE changelog.id = $16
                RETURNING *"#)
            .bind(update.timestamp).bind(update.profile_number).bind(update.score).bind(update.map_id) 
            .bind(update.demo_id).bind(update.banned).bind(update.youtube_id).bind(update.coop_id)
            .bind(update.post_rank).bind(update.pre_rank).bind(update.submission).bind(update.note)
            .bind(update.category_id).bind(update.score_delta).bind(update.verified).bind(update.admin_note)
            .bind(update.id)
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
}

impl ChangelogPage{
    // Gets as many changelog entries as the limit passed.
    // TODO: Base this on time rather than a hard limit??
    pub async fn get_cl_page(pool: &PgPool, limit: i32) -> Result<Option<Vec<ChangelogPage>>>{
        let res = sqlx::query_as::<_, ChangelogPage>(r#" 
                SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned, 
                cl.youtube_id, cl.previous_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
                cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name AS map_name, 
                CASE
                    WHEN u.board_name IS NULL
                        THEN u.steam_name
                    WHEN u.board_name IS NOT NULL
                        THEN u.board_name
                END user_name,
                u.avatar
                FROM "p2boards".changelog AS cl
                INNER JOIN "p2boards".users AS u ON (u.profile_number = cl.profile_number)
                INNER JOIN "p2boards".maps AS map ON (map.steam_id = cl.map_id)
                ORDER BY cl.timestamp DESC NULLS LAST
                LIMIT $1"#)
            .bind(limit)
            .fetch_all(pool)
            .await;
        match res{
            Ok(res) => return Ok(Some(res)),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
    }
    // Handles filtering out changelog by different criteria.
    pub async fn get_cl_page_filtered(pool: &PgPool, params: ChangelogQueryParams) -> Result<Option<Vec<ChangelogPage>>>{
        // TODO: Decide if we want Chapter name
        let mut query_string: String = String::from(r#" 
            SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned, 
            cl.youtube_id, cl.previous_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
            cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name AS map_name,  
            CASE
                WHEN u.board_name IS NULL
                    THEN u.steam_name
                WHEN u.board_name IS NOT NULL
                    THEN u.board_name
            END user_name,
            u.avatar
            FROM "p2boards".changelog AS cl
            INNER JOIN "p2boards".users AS u ON (u.profile_number = cl.profile_number)
            INNER JOIN "p2boards".maps AS map ON (map.steam_id = cl.map_id)
            INNER JOIN "p2boards".chapters AS chapter on (map.chapter_id = chapter.id)"#);
        if !params.coop{
            query_string = format!("{} WHERE chapters.is_multiplayer = False", &query_string);
        } else if !params.sp{
            query_string = format!("{} WHERE chapters.is_multiplayer = True", &query_string);
        }
        if let Some(has_demo) = params.has_demo{
            if has_demo{
                query_string = format!("{} WHERE cl.demo_id IS NOT NULL", &query_string);
            } else{
                query_string = format!("{} WHERE cl.demo_id IS NULL", &query_string);
            }
        }
        if let Some(yt) = params.yt{
            if yt{
                query_string = format!("{} WHERE cl.youtube_id IS NOT NULL", &query_string);
            } else{
                query_string = format!("{} WHERE cl.youtube_id IS NULL", &query_string);
            }
        }
        if let Some(wr_gain) = params.wr_gain{
            if wr_gain{
                query_string = format!("{} WHERE cl.post_rank = 1", &query_string);
            }
        }
        if let Some(chamber) = params.chamber{
            query_string = format!("{} WHERE cl.map_id = {}", &query_string, &chamber);
        }
        if let Some(profile_number) = params.profile_number{
            query_string = format!("{} WHERE cl.profile_number = {}", &query_string, &profile_number);
        }
        //#[allow(irrefutable_let_patterns)]
        if let Some(nick_name) = params.nick_name{
            //eprintln!("{}", nick_name);
            if let Some(profile_numbers) = Users::check_board_name(&pool, nick_name.clone()).await?.as_mut(){
                if profile_numbers.len() == 1{
                    query_string = format!("{} WHERE cl.profile_number = '{}'", &query_string, &profile_numbers[0].to_string());
                } else{
                    query_string = format!("{} WHERE cl.profile_number = '{}'", &query_string, &profile_numbers[0].to_string());
                    profile_numbers.remove(0);
                    for num in profile_numbers.iter(){
                        query_string = format!("{} OR cl.profile_number = '{}'", &query_string, num);
                    }
                }
            }
            else{
                // TODO: Construct an Error
            }
        }
        //TODO: Maybe allow for custom order params????
        query_string = format!("{} ORDER BY cl.timestamp DESC NULLS LAST", &query_string);
        if let Some(limit) = params.limit{
            query_string = format!("{} LIMIT {}", &query_string, limit);
        } else{ // Default limit
            query_string = format!("{} LMIT 1000", &query_string)
        }
        //eprintln!("{}", query_string);
        let res = sqlx::query_as::<_, ChangelogPage>(&query_string).fetch_all(pool).await;
        match res{
            Ok(changelog_filtered) => Ok(Some(changelog_filtered)),
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::Error::new(e).context("Error with SP Maps"))
            },
        }
        // Ok(Some(res))
    }
}

impl Demos {
    pub async fn insert_demo(pool: &PgPool, demo: DemoInsert) -> Result<i64>{
        let mut res: i64 = 0; 
        let query = sqlx::query(r#"
                INSERT INTO "p2boards".demos 
                (file_id, partner_name, parsed_successfully, sar_version, cl_id) VALUES 
                ($1, $2, $3, $4, $5)
                RETURNING id"#)
            .bind(demo.file_url).bind(demo.partner_name).bind(demo.parsed_successfully).bind(demo.sar_version).bind(demo.cl_id)
            .map(|row: PgRow|{res = row.get(0)})
            .fetch_one(pool)
            .await?;
            Ok(res)
    }
}