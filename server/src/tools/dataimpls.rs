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



// impl SpPreview{
//     // TODO: Improve error handling.
//     // TODO: Check on distinct_on support or MySQL.
//     // WHY IS DISTINCT_ON ONLY SUPPORTED FOR POSTGRES?
//     /// Grabs top 40 times in an SP map. This is a crutch to avoid filtering out too many times for duplicates from one user.
//     /// We do this rather than using the `DISTINCT_ON` method in diesel & MySQL as it's not currently supported by Diesel, the hope is that this is changed in the future.
//     pub fn show(conn: &PgPool, mapid: String) -> Vec<SpPreview>{
//         all_changelogs
//             .inner_join(all_users)
//             .select((changelog::map_id, changelog::profile_number, changelog::score, changelog::youtube_id, changelog::category, usersnew::boardname, usersnew::steamname))
//             .filter(changelog::map_id.eq(mapid))
//             .filter(changelog::banned.eq(0))
//             .filter(usersnew::banned.eq(0))
//             .filter(changelog::category.eq("any%".to_string()))
//             .order(changelog::score.asc())
//             .limit(40)
//             .load::<SpPreview>(conn)
//             .expect("Error loading map previews for SP.")
//     } 
// }

// impl SpPreviews{
//     /// Makes a call to grab all `map_ids`, grabs the map name off a call to `Map::get_name()` and
//     /// calls`SpPreview::show()` for all 60 sp maps and filters out top 7 for distinct runners.
//     // Top 40 to handle duplicates to generate top 7, that number should be safe right?
//     pub fn show(conn: &PgPool) -> Result<Option<Vec<SpPreviews>>, diesel::result::Error>{
//         let map_id_vec = Map::all_sp_mapids(&conn);
//         let mut vec_final = Vec::new();
//         for mapid in map_id_vec.iter(){
//             let vec_temp = SpPreview::show(&conn, mapid.to_string());
//             let mut vec_filtered = Vec::new();
//             let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(50);
//             // Yes I know this is stupid, diesel.rs doesn't support distinct_on clauses with MySQL...
//             for entry in vec_temp{
//                 match remove_dups.insert(entry.profile_number.clone(), 1){
//                     Some(_) => (),
//                     _ =>{
//                         vec_filtered.push(entry);
//                     }
//                 }
//             }
//             vec_filtered.truncate(7);
//             vec_final.push(SpPreviews{ map_name: Map::get_name(&conn, mapid.to_string()), scores: vec_filtered})
//         }
//         Ok(Some(vec_final))
//     }
// }

// impl SpBanned{
//     pub fn show(conn: &PgPool, mapid: String) -> Result<Vec<SpBanned>, diesel::result::Error>{
//         let changelog_entries = all_changelogs
//             .select((changelog::profile_number, changelog::score))
//             .filter(changelog::banned.eq(1))
//             .filter(changelog::map_id.eq(mapid))
//             .order(changelog::score.asc())
//             .load::<SpBanned>(conn);
//         if let Ok(changelog_entries) = changelog_entries{
//             return Ok(changelog_entries);
//         } else{
//             return Err(diesel::result::Error::NotFound);
//         }
//     }
// }

// impl CoopBanned{
//     pub fn show(conn: &PgPool, mapid: String) ->Result<Vec<CoopBanned>, diesel::result::Error>{
//         let coopbundled_entries = all_coops
//             .select((coopbundled::profile_number1, coopbundled::profile_number2, coopbundled::score))
//             .filter(coopbundled::map_id.eq(mapid))
//             .filter(coopbundled::banned.eq(1))
//             .order(coopbundled::score.asc())
//             .load::<CoopBanned>(conn);
//         if let Ok(coopbundled_entries) = coopbundled_entries{
//             return Ok(coopbundled_entries);
//         } else{
//             return Err(diesel::result::Error::NotFound);
//         }
//     }
// }

// impl Map{
//     /// Grabs the map at a given steam_id
//     pub fn show(conn: &PgPool, id: String) -> Vec<Map> {
//         all_maps
//             .filter(maps::steam_id.eq(id))
//             .load::<Map>(conn)
//             .expect("Error Loading Maps")
//     }
//     /// Grabs all steam_ids for single player
//     pub fn all_sp_mapids(conn: &PgPool) -> Vec<String> {
//         all_maps
//             .select(maps::steam_id)
//             .filter(maps::is_coop.eq(0))
//             .load::<String>(conn)
//             .expect("Error loading SP maps")
//     }
//     /// Grabs all steam_ids for Cooperative
//     pub fn all_coop_mapids(conn: &PgPool) -> Vec<String> {
//         all_maps
//             .select(maps::steam_id)
//             .filter(maps::is_coop.eq(1))
//             .load::<String>(conn)
//             .expect("Error loading SP maps")
//     }
//     /// Grabs all map info and loads it into Map vectors
//     pub fn all(conn: &PgPool) -> Vec<Map> {
//         all_maps
//             .order(maps::id.desc())
//             .load::<Map>(conn)
//             .expect("Error loading all maps")
//     }
//     /// Grabs the map name for the map at the given steam_id
//     pub fn get_name(conn: &PgPool, mapid: String) -> Option<String>{
//         all_maps
//             .select((maps::name))
//             .filter(maps::steam_id.eq(mapid))
//             .first(conn)
//             .expect("Cannot find mapname")
//     }
// }

// impl Usersnew{
//     /// Gets the boardname, steamname and avatar from the database for a specifc user.
//     pub fn show(conn: &PgPool, profilenum: String) -> Result<Option<UserMap>, diesel::result::Error>{
//         let user = all_users
//             .select((usersnew::boardname.nullable(), usersnew::steamname.nullable(), usersnew::avatar.nullable()))
//             .find(profilenum)
//             .first(conn)?;
//         Ok(Some(user))
//     }
//     /// Search function to find if there's a player with a similar boardname in the database.
//     pub fn check_board_name(conn: &PgPool, nickname: String) -> Result<bool, diesel::result::Error>{
//         let user = all_users
//             .filter(usersnew::boardname.like(nickname))
//             .filter(usersnew::banned.eq(0))
//             .get_result::<Usersnew>(conn);
//         // Dear god fix this error handling
//         if let Ok(user) = user{
//             return Ok(true);
//         }
//         else{
//             Ok(false)
//         }
//     }
//     pub fn showbanned(conn: &PgPool) -> Result<Vec<String>, diesel::result::Error>{
//         let user = all_users
//             .select(usersnew::profile_number)
//             .filter(usersnew::banned.eq(1))
//             .load::<String>(conn)?;
//         Ok(user)
//     }
//     pub fn check_banned(conn: &PgPool, profilenum: String) -> Result<bool, diesel::result::Error>{
//         let user = all_users
//             .filter(usersnew::profile_number.eq(profilenum))
//             .get_result::<Usersnew>(conn);
//         if let Ok(user) = user{
//             if user.banned == 1{
//                 return Ok(true)
//             }
//             else{
//                 Ok(false)
//             }
//         }
//         else{
//             return Err(diesel::result::Error::NotFound);
//         }
//     }
// }

// // TODO: Fix this when diesel adds support for aliased queries.
// impl CoopMapPrelude{
//     /// Work-around for lack of alias support, grabs all information except the second user's profile data for the times on a given map.
//     pub fn show(conn: &PgPool, mapid: String) -> Result<Option<Vec<CoopMapPrelude>>, diesel::result::Error>{
//         let map = all_coops
//             .inner_join(all_users)
//             // Horrific work-around, I'm sorry god.
//             .select((
//                 coopbundled::time_gained.nullable(), coopbundled::profile_number1,        coopbundled::profile_number2, 
//                 coopbundled::score,                  coopbundled::is_blue.nullable(),     coopbundled::has_demo1.nullable(), 
//                 coopbundled::has_demo2.nullable(),   coopbundled::youtube_id1.nullable(), coopbundled::youtube_id2.nullable(),
//                 coopbundled::submission1,            coopbundled::submission2, 
//                 coopbundled::note1.nullable(),       coopbundled::note2.nullable(),       coopbundled::category.nullable(), 
//                 usersnew::boardname.nullable(),      usersnew::steamname.nullable(),      usersnew::avatar.nullable()))
//             .filter(coopbundled::map_id.eq(mapid))
//             .filter(coopbundled::banned.eq(0))
//             .filter(usersnew::banned.eq(0))
//             .order(coopbundled::score.asc())
//             .load::<CoopMapPrelude>(conn)?;
//         Ok(Some(map))
//     }
// }

// /// Returns a result wrapped option that wraps an instance of CoopMap
// /// Makes calls to both CoopMapPrelude and individual Usernews.println!
// /// Handles null partners with Option<None>'s.
// // TODO: Look into non-blocking, concurrent alternatives while we're using this work-around.
// impl CoopMap{
//     /// Calls `CoopMapPrelude::show` and `Usersnew::show` to fill in data due to lack of alias support in diesel. Returns all coop map information for a given map.actix_web
//     pub fn show(conn: &PgPool, mapid: String) -> Result<Option<Vec<CoopMap>>, diesel::result::Error>{
//         let coop_prelude = CoopMapPrelude::show(&conn, mapid.clone())?;
//         if let Some(coop_prelude) = coop_prelude {
//             let mut vec_final = Vec::new();
//             // Moving ownership to the for loop iteration
//             for entry in coop_prelude {
//                 let tempstr = &entry.profile_number2;
//                 if(tempstr != "")  {
//                     let user2 = Usersnew::show(&conn, tempstr.to_string())?;
//                     if let Some(user2) = user2{
//                         let tempstruct = CoopMap {time_gained: entry.time_gained, profile_number1: entry.profile_number1, profile_number2: entry.profile_number2, score: entry.score, is_blue: entry.is_blue,
//                             has_demo1: entry.has_demo1, has_demo2: entry.has_demo2, youtube_id1: entry.youtube_id1, youtube_id2: entry.youtube_id2, submission1: entry.submission1, submission2: entry.submission2,
//                             note1: entry.note1, note2: entry.note2, category: entry.category, boardname1: entry.boardname, steamname1: entry.steamname, avatar1: entry.avatar, boardname2: user2.boardname, 
//                             steamname2: user2.steamname, avatar2: user2.avatar};
//                         vec_final.push(tempstruct);
//                     } else{
//                         println!("Unexpected Error");
//                     }
//                 }
//                 else{
//                     let tempstruct = CoopMap {time_gained: entry.time_gained, profile_number1: entry.profile_number1, profile_number2: entry.profile_number2, score: entry.score, is_blue: entry.is_blue,
//                         has_demo1: entry.has_demo1, has_demo2: entry.has_demo2, youtube_id1: entry.youtube_id1, youtube_id2: entry.youtube_id2, submission1: entry.submission1, submission2: entry.submission2,
//                         note1: entry.note1, note2: entry.note2, category: entry.category, boardname1: entry.boardname, steamname1: entry.steamname, avatar1: entry.avatar, boardname2: None, 
//                         steamname2: None, avatar2: None};
//                     vec_final.push(tempstruct);
//                 }
//             }
//             Ok(Some(vec_final))
//         } else{
//             // TODO: FIXME: Awful Error Handling
//             let vec_final = Vec::new();
//             Ok(Some(vec_final))
//         }
//     }
// }

impl SpMap{
    // TODO: Fix this query to be non-ambiguous
    pub async fn get_sp_map(pool: &PgPool, map_id: String) -> Result<Option<Vec<SpMap>>> {
        let query = sqlx::query_as::<_, SpMap>(r#" 
                    SELECT t.timestamp,
                    t.profile_number,
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
                    SELECT DISTINCT ON (changelog.profile_number) *
                    FROM "p2boards".changelog
                    INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
                    WHERE map_id = '47763'
                    AND users.banned = False
                    AND changelog.verified = True
                    AND changelog.banned = False
                    ORDER BY changelog.profile_number, changelog.score ASC
                ) t
                ORDER BY score;"#)
            .bind(map_id)
            .fetch_all(pool)
            .await?;
        Ok(Some(query))
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
//     /// Returns `limit` number of changelog entries. Handle the joining for map and user data.
//     pub fn show(conn: &PgPool, limit: i32) -> Result<Option<Vec<ChangelogPage>>, diesel::result::Error>{ 
//         let cl = all_changelogs
//             .inner_join(all_users)
//             .inner_join(all_maps.on(changelog::map_id.eq(maps::steam_id)))
//             .select((changelog::time_gained.nullable(), changelog::profile_number,
//             changelog::score, changelog::map_id, changelog::wr_gain, 
//             changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
//             changelog::previous_id.nullable(), changelog::id, changelog::coopid.nullable(),
//             changelog::post_rank.nullable(), changelog::pre_rank.nullable(),
//             changelog::submission, changelog::note.nullable(), 
//             changelog::category.nullable(), maps::name, usersnew::boardname.nullable(), 
//             usersnew::steamname.nullable(), usersnew::avatar.nullable()))
//             .order(changelog::time_gained.desc())
//             //.filter(changelog::time_gained.is_not_null())
//             .filter(usersnew::banned.eq(0))
//             .filter(changelog::profile_number.ne("".to_string()))
//             .limit(limit.into())
//             .load::<ChangelogPage>(conn)?;
//         Ok(Some(cl))
//     }
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