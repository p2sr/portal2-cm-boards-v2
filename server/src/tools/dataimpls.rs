#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error, HttpRequest, Responder};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row, PgPool};
use futures::future::{ready, Ready};
use anyhow::Result;
use chrono::NaiveDateTime;

use crate::tools::datamodels::*;

// impl CoopPreviewPrelude{
//     /// Only used internally by the `CoopPreview::show` method.
//     /// Grabs the top 40 times on a Coop map. Used as a prelude because of limitations with diesel and aliases.
//     pub fn show(conn: &PgPool, mapid: String) -> Vec<CoopPreviewPrelude>{
//         all_coops
//             .inner_join(all_users)
//             .select((coopbundled::map_id, coopbundled::profile_number1, coopbundled::profile_number2, coopbundled::score, coopbundled::youtube_id1, coopbundled::youtube_id2, coopbundled::category, usersnew::boardname, usersnew::steamname))
//             .filter(coopbundled::map_id.eq(mapid))
//             .filter(coopbundled::banned.eq(0))
//             .filter(usersnew::banned.eq(0))
//             .filter(coopbundled::category.eq("any%".to_string()))
//             .order(coopbundled::score.asc())
//             .limit(40)
//             .load::<CoopPreviewPrelude>(conn)
//             .expect("Error loading map previews for SP.")
//     }
// }

// impl CoopPreview{
//     /// Only used internally by the `CoopPreviews::show` method.
//     /// Joins user information for a given set of scores for a map.
//     pub fn show(conn: &PgPool, mapid: String) -> Result<Option<Vec<CoopPreview>>, diesel::result::Error>{
//         let coop_prelude = CoopPreviewPrelude::show(&conn, mapid.clone());
//         let mut vec_joined = Vec::new();
//         // Moving ownership to the for loop iteration
//         for entry in coop_prelude {
//             let tempstr = &entry.profile_number2;
//             if(tempstr != ""){
//                 let user2 = Usersnew::show(&conn, tempstr.to_string())?;
//                 if let Some(user2) = user2{
//                     let tempstruct = CoopPreview {map_id: entry.map_id, profile_number1: entry.profile_number1,
//                          profile_number2: entry.profile_number2, score: entry.score, youtube_id1: entry.youtube_id1, 
//                          youtube_id2: entry.youtube_id2, category: entry.category, boardname1: entry.boardname, 
//                          steamname1: entry.steamname, boardname2: user2.boardname, steamname2: user2.steamname};
//                     vec_joined.push(tempstruct)
//                 } else{
//                     println!("Unexpected Error.")
//                 }
//             } else{
//                 let tempstruct = CoopPreview {map_id: entry.map_id, profile_number1: entry.profile_number1,
//                     profile_number2: entry.profile_number2, score: entry.score, youtube_id1: entry.youtube_id1, 
//                     youtube_id2: entry.youtube_id2, category: entry.category, boardname1: entry.boardname, 
//                     steamname1: entry.steamname, boardname2: None, steamname2: None};
//                 vec_joined.push(tempstruct)
//             }
//         }
//         Ok(Some(vec_joined))
//     }
// }

// impl CoopPreviews{
//     /// Calls two internal functions to gather the necessary information, and truncates excess information.
//     /// Uses manual filtering through hashmaps to eliminate duplicate times by players in accordance to how cooperative handles "carrying".
//     /// The logic is slightly more complicated, and nearly identical to the logic in the handler for coop maps.
//     pub fn show(conn: &PgPool) -> Result<Option<Vec<CoopPreviews>>, diesel::result::Error>{
//         let map_id_vec = Map::all_coop_mapids(&conn);
//         let mut vec_final = Vec::new();
//         for mapid in map_id_vec.iter(){
//             let vec_temp = CoopPreview::show(&conn, mapid.to_string())?;
//             if let Some(vec_temp) = vec_temp{
//                 let mut vec_filtered = Vec::new();
//                 let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(100);
//                 for entry in vec_temp{
//                     match remove_dups.insert(entry.profile_number1.clone(), 1){
//                         Some(_) => match remove_dups.insert(entry.profile_number2.clone(), 1){
//                             Some(_) => (),
//                             _ => vec_filtered.push(entry),
//                         }
//                         _ => match remove_dups.insert(entry.profile_number2.clone(), 1){
//                             Some(_) => vec_filtered.push(entry),
//                             _ => vec_filtered.push(entry),
//                         }    
//                     }
//                 }
//                 vec_filtered.truncate(7);
//                 vec_final.push(CoopPreviews{ map_name: Map::get_name(&conn, mapid.to_string()), scores: vec_filtered});
//             } else{println!("Unexpected Error");}
//         }
//         Ok(Some(vec_final))
//     }
// }


impl SpPreviews{
    /// Gets preview information for top 7 on an SP Map.
    pub async fn get_sp_previews(pool: &PgPool, map_id: String) -> Result<SpPreviews>{
        let query = sqlx::query_as::<_, SpPreview>(r#"
                SELECT t.CL_profile_number, t.score, t.youtube_id, t.category_id,
                CASE
                    WHEN t.board_name IS NULL
                        THEN t.steam_name
                    WHEN t.board_name IS NOT NULL
                        THEN t.board_name
                    END user_name
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
            .await?;
        Ok(SpPreviews{
            map_id: map_id,
            map_data: query, 
        })
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
            .await?;   
        Ok(res)
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
        let res = sqlx::query!(r#"
                SELECT maps.steam_id FROM "p2boards".maps
                    INNER JOIN "p2boards".chapters ON (maps.chapter_id = chapters.id)
                    WHERE chapters.is_multiplayer = $1
                "#)
            .bind(is_mp)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
    /// Returns the map name for a given steam_id.
    pub async fn get_map_name(pool: &PgPool, map_id: String) -> Result<Option<String>>{
        let res = sqlx::query!(r#"SELECT maps.name FROM "p2boards".maps WHERE maps.steam_id = $1"#)
            .bind(map_id)
            .fetch_one(pool)
            .await?;
        Ok(Some(res))
    }
}

impl Users{
    /// Pattern match on a given string to find similar names (supports board/steam names). 
    pub async fn check_board_name(pool: &PgPool, nick_name: String) -> Result<Option<Vec<String>>>{ // TODO: Check return type of 0 results more carefully
        let query_nn = format!("%{}%", &nick_name);
        let res = sqlx::query!(r#"
                SELECT users.profile_number FROM "p2boards".users
                WHERE 
                    CASE
                        WHEN users.board_name IS NULL
                            THEN users.steam_name LIKE $1
                        WHEN users.board_name IS NOT NULL
                            THEN users.board_name LIKE $1
                    END
                "#) 
            .bind(query_nn)
            .fetch_all(pool)
            .await?;
        Ok(Some(res))
    }
    /// Returns a list of all banned players profile_numbers.
    pub async fn get_banned(pool: &PgPool) -> Result<Vec<String>>{
        let res = sqlx::query!(r#"SELECT users.profile_number FROM "p2boards".users WHERE users.banned = True"#)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
    /// Returns a boolean based on if the profile number passed is banned or not.
    pub async fn check_banned(pool: &PgPool, profile_number: String) -> Result<bool>{
        let res = sqlx::query!(r#"SELECT users.banned FROM "p2boards".users WHERE users.profile_number = $1"#)
            .fetch_one(pool)
            .await?;
        Ok(res)
    }
}

impl CoopMap{
    // TODO: Check to make sure this is filtered when returned (I think it is).
    pub async fn get_coop_map_page(pool: &PgPool, map_id: String) -> Result<Option<Vec<CoopMap>>> {
        let res = sqlx::query_as::<_, CoopMap>(r#"
                SELECT  c1.timestamp, 
                    c1.score, c1.note, c2.note,
                    CASE 
                    WHEN p1.board_name IS NULL
                        THEN p1.steam_name
                    WHEN p1.board_name IS NOT NULL
                        THEN p1.board_name
                    END p1_username, 
                    CASE 
                    WHEN p2.board_name IS NULL
                        THEN p2.steam_name
                    WHEN p2.board_name IS NOT NULL
                        THEN p2.board_name
                    END p2_username ,
                    c1.profile_number, c2.profile_number, c1.demo_id, c2.demo_id, c1.youtube_id,
                    c2.youtube_id, c1.submission, c2.submission, c1.category_id, c2.category_id, p1.avatar, p2.avatar
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
                ORDER BY score ASC;
                "#)
            .bind(map_id)
            .fetch_all(pool)
            .await?;
        Ok(Some(res))
    }
}

impl SpMap{
    pub async fn get_sp_map_page(pool: &PgPool, map_id: String) -> Result<Option<Vec<SpMap>>> {
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
                        WHERE map_id = '47763'
                        AND users.banned = False
                        AND changelog.verified = True
                        AND changelog.banned = False
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score"#) 
            .bind(map_id)
            .fetch_all(pool)
            .await?;
        Ok(Some(res))
    }
}

// Implementation of Actix::Responder for Changelog struct so we can return Changelog from action handler
impl Responder for Changelog {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementations of associated functions for Changelog
impl Changelog{
    /// Check for if a given score already exists in the database, but is banned. Used for the auto-updating from Steam leaderboards.
    /// Returns `true` if there is a value found, `false` if no value or error.
    pub async fn check_banned_scores(pool: &PgPool, map_id: String, score: i32, profile_number: String) -> Result<bool>{
        // We don't care about the result, we only care if there is a result.
        let query = sqlx::query!(r#" 
                SELECT * 
                FROM "p2boards".changelog
                WHERE changelog.score = $1
                AND changelog.map_id = $2
                AND changelog.profile_number = $3
                AND changelog.banned = $4"#)
            .bind(score)
            .bind(map_id)
            .bind(profile_number)
            .bind(false);
        let res = query.fetch_one(pool).await;
        match res{
            Ok(_) => return Ok(true),
            Err(_) => return Ok(false),
        }
    }
    // Returns a vec of changelog for a user's PB history on a given SP map.
    pub async fn sp_pb_history(pool: &PgPool, map_id: String, profile_number: String) -> Result<Option<Vec<Changelog>>>{
        let query = sqlx::query_as::<_, Changelog>(r#" 
                SELECT * 
                FROM "p2boards".changelog
                WHERE changelog.profile_number = $1
                AND changelog.map_id = $2
                ORDER BY changelog.score ASC"#)
            .bind(profile_number)
            .bind(map_id);
        let res: Vec<Changelog> = query.fetch_all(pool).await?;
        Ok(Some(res))
    }
    /// Insert a new changelog entry.
    pub async fn insert_changelog(pool: &PgPool, cl: ChangelogInsert) -> Result<i64>{
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
            .map(|row: PgRow|{
                res = row.get(0)
            })
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
    pub async fn get_cl_page(pool: &PgPool, limit: i32) -> Result<Option<Vec<ChangelogPage>>>{
        let query = sqlx::query_as::<_, ChangelogPage>(r#" 
                SELECT cl.id, cl.timestamp, cl.profile_number, cl.score, cl.map_id, cl.demo_id, cl.banned, 
                cl.youtube_id, cl.coop_id, cl.post_rank, cl.pre_rank, cl.submission, cl.note,
                cl.category_id, cl.score_delta, cl.verified, cl.admin_note, map.name, 
                CASE
                    WHEN user.board_name IS NULL
                        THEN user.steam_name
                    WHEN user.board_name IS NOT NULL
                        THEN user.board.name
                END user_name, user.avatar
                FROM "p2boards".changelog AS cl
                INNER JOIN "p2boards".users AS user ON (user.profile_number = cl.profile_number
                INNER JOIN "p2boards".maps AS map ON (map.steam_id = cl.map_id)
                ORDER BY cl.timestamp DESC
                LIMIT $1"#)
            .bind(limit)
            .fetch_all(pool)
            .await?;
            
        Ok(Some(vec![]))
    }

    // pub async fn get_cl_page_filtered(pool: &PgPool, ) -> Result<Option<Vec<ChangelogPage>>>{

    // }
}

// impl ChangelogPage{
//     // TODO: Make this a struct dear god...
//     /// Filtering options for the changelog through any of the options passed. For more information, checkout `ChangelogQueryParams` in `datamodels.rs`
//     pub fn show_filtered(conn: &PgPool, nickname: Option<String>, 
//         profilenumber: Option<String>, chamber: Option<String>,  sp: Option<i32>, 
//         coop: Option<i32>, wrgain: Option<i32>, hasdemo: Option<i32>, hasvideo: Option<i32>,
//         limit: i32) ->  Result<Option<Vec<ChangelogPage>>, diesel::result::Error>
//         {
//         let mut query = all_changelogs
//         .inner_join(all_users)
//         .inner_join(all_maps.on(changelog::map_id.eq(maps::steam_id)))
//         .select((changelog::time_gained.nullable(), changelog::profile_number,
//         changelog::score, changelog::map_id, changelog::wr_gain, 
//         changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
//         changelog::previous_id.nullable(), changelog::id, changelog::coopid.nullable(),
//         changelog::post_rank.nullable(), changelog::pre_rank.nullable(),
//         changelog::submission, changelog::note.nullable(), 
//         changelog::category.nullable(), maps::name, usersnew::boardname.nullable(), 
//         usersnew::steamname.nullable(), usersnew::avatar.nullable()))
//         .order(changelog::time_gained.desc())
//         .filter(usersnew::banned.eq(0))
//         .into_boxed();
//         if let Some(sp) = sp{
//             query = query.filter(maps::is_coop.ne(sp));
//         }
//         else if let Some(coop) = coop{
//             query = query.filter(maps::is_coop.eq(coop));
//         }
//         if let Some(hasdemo) = hasdemo{
//             query = query.filter(changelog::has_demo.eq(1));
//         }
//         if let Some(hasvideo) = hasvideo{
//             query = query.filter(changelog::youtube_id.is_not_null());
//         }
//         if let Some(wrgain) = wrgain{
//             query = query.filter(changelog::wr_gain.eq(1));
//         }
//         if let Some(chamber) = chamber{
//             query = query.filter(changelog::map_id.eq(chamber));
//         }
//         if let Some(profilenumber) = profilenumber{
//             query = query.filter(changelog::profile_number.eq(profilenumber));
//         }
//         #[allow(irrefutable_let_patterns)]
//         if let Some(nickname) = nickname{
//             if let namecheck = Usersnew::check_board_name(&conn, nickname.clone()){
//                 query = query.filter(usersnew::boardname.eq(nickname.clone()));
//             }else{
//                 query = query.filter(usersnew::steamname.eq(nickname));
//             }
//         }
//         let result = query.limit(limit.into())
//             .load::<ChangelogPage>(conn)?;
//         Ok(Some(result))
//     }
// }