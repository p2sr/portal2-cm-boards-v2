#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error};

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::schema::{changelog, chapters, coopbundled, maps, scores, usersnew};
use crate::schema::changelog::dsl::changelog as all_changelogs;
use crate::schema::usersnew::dsl::usersnew as all_users;
use crate::schema::maps::dsl::maps as all_maps;
use crate::schema::coopbundled::dsl::coopbundled as all_coops;

use crate::db::DbPool;
/// Structs are generated off the database (using deisel_ext) and modified to be used to store query data.
#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "changelog"]
pub struct Changelog {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub banned: i32,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub id: i32,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "chapters"]
pub struct Chapter {
    pub id: u32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "coopbundled"]
pub struct Coopbundled {
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub banned: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub previous_id1: Option<i32>,
    pub previous_id2: Option<i32>,
    pub changelogid1: i32,
    pub changelogid2: i32,
    pub id: i32,
    pub post_rank1: Option<i32>,
    pub post_rank2: Option<i32>,
    pub pre_rank1: Option<i32>,
    pub pre_rank2: Option<i32>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[table_name = "maps"]
pub struct Map {
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: Option<String>,
    pub type_: String,
    pub chapter_id: Option<u32>,
    pub is_coop: i32,
    pub is_public: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[primary_key(changelog_id)]
#[table_name = "scores"]
pub struct Score {
    pub profile_number: String,
    pub map_id: String,
    pub changelog_id: i32,
}

#[derive(Serialize, Queryable, Debug, Clone, Identifiable)]
#[primary_key(profile_number)]
#[table_name = "usersnew"]
pub struct Usersnew {
    pub profile_number: String,
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub banned: i32,
    pub registered: i32,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
}


/// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SPMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub has_demo: Option<i32>,
    pub youtube_id: Option<String>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// This struct is a work-around for the issues with aliased queries in diesel, ideally this would be scrapped for an aliased join on usersnew
/// so we could grab both sets of usersnew information in one query
/// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMapPrelude{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// This is the big brother struct for the prelude to handle all of the data. The overhead on copy all the data is relatively small, 
/// but ideally we would only need this and not the prelude.
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMap{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub is_blue: Option<i32>,
    pub has_demo1: Option<i32>,
    pub has_demo2: Option<i32>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: i32,
    pub submission2: i32,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category: Option<String>,
    pub boardname1: Option<String>,
    pub steamname1: Option<String>,
    pub avatar1: Option<String>,
    pub boardname2: Option<String>,
    pub steamname2: Option<String>,
    pub avatar2: Option<String>,
}

/// Grabs just the essential user information to aid in filling in the CoopMapPrelude
// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UserMap{
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

/// SpPreview and SpPreviews generate the time information displayed on the /sp route. 
// These two work together to grab all that data from the database (only what's necessary)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SpPreview{
    pub map_id: String,
    pub profile_number: String,
    pub score: i32,
    pub youtube_id: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
}

/// Preview the top 7 for all SP maps (any%)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SpPreviews{
    pub map_name: Option<String>,
    pub scores: Vec<SpPreview>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreviewPrelude{
    pub map_id: String,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category: Option<String>, 
    pub boardname: Option<String>,
    pub steamname: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreview{
    pub map_id: String,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category: Option<String>, 
    pub boardname1: Option<String>,
    pub steamname1: Option<String>,
    pub boardname2: Option<String>,
    pub steamname2: Option<String>,
}


#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopPreviews{
    pub map_name: Option<String>,
    pub scores: Vec<CoopPreview>,
}

impl CoopPreviewPrelude{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Vec<CoopPreviewPrelude>{
        all_coops
            .inner_join(all_users)
            .select((coopbundled::map_id, coopbundled::profile_number1, coopbundled::profile_number2, coopbundled::score, coopbundled::youtube_id1, coopbundled::youtube_id2, coopbundled::category, usersnew::boardname, usersnew::steamname))
            .filter(coopbundled::map_id.eq(mapid))
            .filter(coopbundled::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .filter(coopbundled::category.eq("any%".to_string()))
            .order(coopbundled::score.asc())
            .limit(40)
            .load::<CoopPreviewPrelude>(conn)
            .expect("Error loading map previews for SP.")
    }
}

impl CoopPreview{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<CoopPreview>>, diesel::result::Error>{
        let coop_prelude = CoopPreviewPrelude::show(&conn, mapid.clone());
        let mut vec_joined = Vec::new();
        // Moving ownership to the for loop iteration
        for entry in coop_prelude {
            let tempstr = &entry.profile_number2;
            if(tempstr != ""){
                let user2 = Usersnew::show(&conn, tempstr.to_string())?;
                if let Some(user2) = user2{
                    let tempstruct = CoopPreview {map_id: entry.map_id, profile_number1: entry.profile_number1,
                         profile_number2: entry.profile_number2, score: entry.score, youtube_id1: entry.youtube_id1, 
                         youtube_id2: entry.youtube_id2, category: entry.category, boardname1: entry.boardname, 
                         steamname1: entry.steamname, boardname2: user2.boardname, steamname2: user2.steamname};
                    vec_joined.push(tempstruct)
                } else{
                    println!("Unexpected Error.")
                }
            } else{
                let tempstruct = CoopPreview {map_id: entry.map_id, profile_number1: entry.profile_number1,
                    profile_number2: entry.profile_number2, score: entry.score, youtube_id1: entry.youtube_id1, 
                    youtube_id2: entry.youtube_id2, category: entry.category, boardname1: entry.boardname, 
                    steamname1: entry.steamname, boardname2: None, steamname2: None};
                vec_joined.push(tempstruct)
            }
        }
        Ok(Some(vec_joined))
    }
}

impl CoopPreviews{
    pub fn show(conn: &MysqlConnection) -> Result<Option<Vec<CoopPreviews>>, diesel::result::Error>{
        let map_id_vec = Map::all_coop_mapids(&conn);
        let mut vec_final = Vec::new();
        for mapid in map_id_vec.iter(){
            let vec_temp = CoopPreview::show(&conn, mapid.to_string())?;
            if let Some(vec_temp) = vec_temp{
                let mut vec_filtered = Vec::new();
                let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(100);
                for entry in vec_temp{
                    match remove_dups.insert(entry.profile_number1.clone(), 1){
                        Some(_) => match remove_dups.insert(entry.profile_number2.clone(), 1){
                            Some(_) => (),
                            _ => vec_filtered.push(entry),
                        }
                        _ => match remove_dups.insert(entry.profile_number2.clone(), 1){
                            Some(_) => vec_filtered.push(entry),
                            _ => vec_filtered.push(entry),
                        }    
                    }
                }
                vec_filtered.truncate(7);
                vec_final.push(CoopPreviews{ map_name: Map::get_name(&conn, mapid.to_string()), scores: vec_filtered});
            } else{println!("Unexpected Error");}
        }
        Ok(Some(vec_final))
    }
}

/// impl block for SpPreview
    /// methods:
        /// show(conn: &MysqlConnection, mapid: String)
            /// Grabs 40 times to be filtered (thanks for no distinct_on diesel...) for use on the /sp route.
            /// Returns a vector of SpPreview (40)
impl SpPreview{
    // Maybe improve error handling???? Probably not lol
    // WHY IS DISTINCT_ON ONLY SUPPORTED FOR POSTGRES?
    pub fn show(conn: &MysqlConnection, mapid: String) -> Vec<SpPreview>{
        all_changelogs
            .inner_join(all_users)
            .select((changelog::map_id, changelog::profile_number, changelog::score, changelog::youtube_id, changelog::category, usersnew::boardname, usersnew::steamname))
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .filter(changelog::category.eq("any%".to_string()))
            .order(changelog::score.asc())
            .limit(40)
            .load::<SpPreview>(conn)
            .expect("Error loading map previews for SP.")
    } 
}

/// impl block for SpPreview
    /// methods:
        /// show(conn: &MysqlConnection)
            /// Makes a call to grab all map_ids, grabs the map name off a call to Map::get_name()
            /// Runs through the SpPreview::show() for all 60 sp maps, and filters out top 7 for distinct runners (out of 40, that number should be safe right?)
            /// Returns a vector of SpPreviews, wrapped in an option, wrapped in a result.
impl SpPreviews{
    pub fn show(conn: &MysqlConnection) -> Result<Option<Vec<SpPreviews>>, diesel::result::Error>{
        let map_id_vec = Map::all_sp_mapids(&conn);
        let mut vec_final = Vec::new();
        for mapid in map_id_vec.iter(){
            let vec_temp = SpPreview::show(&conn, mapid.to_string());
            let mut vec_filtered = Vec::new();
            let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(50);
            // Yes I know this is stupid, diesel.rs doesn't support distinct_on clauses with MySQL...
            for entry in vec_temp{
                match remove_dups.insert(entry.profile_number.clone(), 1){
                    Some(_) => (),
                    _ =>{
                        vec_filtered.push(entry);
                    }
                }
            }
            vec_filtered.truncate(7);
            vec_final.push(SpPreviews{ map_name: Map::get_name(&conn, mapid.to_string()), scores: vec_filtered})
        }
        Ok(Some(vec_final))
    }
}
/// impl block for SpPreview
    /// methods:
        /// show(conn: &MysqlConnection, id: String)
            /// Grabs the map at a given steam_id
            /// Returns a vector of maps
        /// all_sp_mapids(conn: &MysqlConnection)
            /// Grabs all steam_ids for single player
            /// Returns a vector of strings
        /// all_coop_mapids(conn: &MysqlConnection)
            /// Grabs all steam_ids for Cooperative
            /// Returns a vector of strings
        /// all(conn: &MysqlConnection)
            /// Grabs all map info and loads it into Map vectors
            /// Returns a vector of Maps
        /// get_name(conn: &MysqlConnection, mapid: String)
            /// Grabs the map name for the map at the given steam_id
            /// Returns a Option wrapped String
impl Map{
    pub fn show(conn: &MysqlConnection, id: String) -> Vec<Map> {
        all_maps
            .filter(maps::steam_id.eq(id))
            .load::<Map>(conn)
            .expect("Error Loading Maps")
    }
    pub fn all_sp_mapids(conn: &MysqlConnection) -> Vec<String> {
        all_maps
            .select(maps::steam_id)
            .filter(maps::is_coop.eq(0))
            .load::<String>(conn)
            .expect("Error loading SP maps")
    }
    pub fn all_coop_mapids(conn: &MysqlConnection) -> Vec<String> {
        all_maps
            .select(maps::steam_id)
            .filter(maps::is_coop.eq(1))
            .load::<String>(conn)
            .expect("Error loading SP maps")
    }
    pub fn all(conn: &MysqlConnection) -> Vec<Map> {
        all_maps
            .order(maps::id.desc())
            .load::<Map>(conn)
            .expect("Error loading all maps")
    }
    pub fn get_name(conn: &MysqlConnection, mapid: String) -> Option<String>{
        all_maps
            .select((maps::name))
            .filter(maps::steam_id.eq(mapid))
            .first(conn)
            .expect("Cannot find mapname")
    }
}

/// impl block for Usersnew
    /// methods:
        /// show(conn: &MysqlConnection, profile_number: String)
            /// Returns a result wrapped option that wraps an instance of UserMap (I know, redundant...)
impl Usersnew{
    pub fn show(conn: &MysqlConnection, profilenum: String) -> Result<Option<UserMap>, diesel::result::Error>{
        let user = all_users
            .select((usersnew::boardname.nullable(), usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .find(profilenum)
            .first(conn)?;
        Ok(Some(user))
    }
}

/// impl block for CoopMapPrelude
    /// methods:
        /// show(conn: &MysqlConnection, map_id: String)
            /// Returns a result wrapped option that wraps an instance of CoopMapPrelude, used to grab the initial join for coop map pages
// TODO: Fix this when diesel adds support for aliased queries.
impl CoopMapPrelude{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<CoopMapPrelude>>, diesel::result::Error>{
        let map = all_coops
            .inner_join(all_users)
            // Horrific work-around, I'm sorry god.
            .select((
                coopbundled::time_gained.nullable(), coopbundled::profile_number1,        coopbundled::profile_number2, 
                coopbundled::score,                  coopbundled::is_blue.nullable(),     coopbundled::has_demo1.nullable(), 
                coopbundled::has_demo2.nullable(),   coopbundled::youtube_id1.nullable(), coopbundled::youtube_id2.nullable(),
                coopbundled::submission1,            coopbundled::submission2, 
                coopbundled::note1.nullable(),       coopbundled::note2.nullable(),       coopbundled::category.nullable(), 
                usersnew::boardname.nullable(),      usersnew::steamname.nullable(),      usersnew::avatar.nullable()))
            .filter(coopbundled::map_id.eq(mapid))
            .filter(coopbundled::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(coopbundled::score.asc())
            .load::<CoopMapPrelude>(conn)?;
        Ok(Some(map))
    }
}

/// impl block for CoopMap
    /// methods:
        /// show(conn: &MysqlConnection, map_id: String) 
            /// Returns a result wrapped option that wraps an instance of CoopMap
            /// Makes calls to both CoopMapPrelude and individual Usernews.println!
            /// Handles null partners with Option<None>'s.
// TODO: Look into non-blocking, concurrent alternatives while we're using this work-around.
impl CoopMap{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<CoopMap>>, diesel::result::Error>{
        let coop_prelude = CoopMapPrelude::show(&conn, mapid.clone())?;
        if let Some(coop_prelude) = coop_prelude {
            let mut vec_final = Vec::new();
            // Moving ownership to the for loop iteration
            for entry in coop_prelude {
                let tempstr = &entry.profile_number2;
                if(tempstr != "")  {
                    let user2 = Usersnew::show(&conn, tempstr.to_string())?;
                    if let Some(user2) = user2{
                        let tempstruct = CoopMap {time_gained: entry.time_gained, profile_number1: entry.profile_number1, profile_number2: entry.profile_number2, score: entry.score, is_blue: entry.is_blue,
                            has_demo1: entry.has_demo1, has_demo2: entry.has_demo2, youtube_id1: entry.youtube_id1, youtube_id2: entry.youtube_id2, submission1: entry.submission1, submission2: entry.submission2,
                            note1: entry.note1, note2: entry.note2, category: entry.category, boardname1: entry.boardname, steamname1: entry.steamname, avatar1: entry.avatar, boardname2: user2.boardname, 
                            steamname2: user2.steamname, avatar2: user2.avatar};
                        vec_final.push(tempstruct);
                    } else{
                        println!("Unexpected Error");
                    }
                }
                else{
                    let tempstruct = CoopMap {time_gained: entry.time_gained, profile_number1: entry.profile_number1, profile_number2: entry.profile_number2, score: entry.score, is_blue: entry.is_blue,
                        has_demo1: entry.has_demo1, has_demo2: entry.has_demo2, youtube_id1: entry.youtube_id1, youtube_id2: entry.youtube_id2, submission1: entry.submission1, submission2: entry.submission2,
                        note1: entry.note1, note2: entry.note2, category: entry.category, boardname1: entry.boardname, steamname1: entry.steamname, avatar1: entry.avatar, boardname2: None, 
                        steamname2: None, avatar2: None};
                    vec_final.push(tempstruct);
                }
            }
            Ok(Some(vec_final))
        } else{
            //FIXME: Awful Error Handling
            let vec_final = Vec::new();
            Ok(Some(vec_final))
        }
    }
}
/// impl block for SPMap
    /// methods:
        /// show(conn: &MysqlConnection, map_id: String) 
            /// Returns a result wrapped option that wraps an instance of SPMap
            /// Selects the necessary information for the maps page, filters out banned times and users
impl SPMap{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<SPMap>>, diesel::result::Error>{
        let map = all_changelogs            
            .inner_join(all_users)
            .select((changelog::time_gained.nullable(), changelog::profile_number, changelog::score, 
                changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
                changelog::submission, changelog::note.nullable(), 
                changelog::category.nullable(), usersnew::boardname.nullable(), 
                usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(changelog::score.asc())
            .load::<SPMap>(conn)?;
        // Wrapping the vector in a result and an option (not necessary but good practice)
        Ok(Some(map))
    }
}


// Test function to grab 50 most recent changelog entries
impl Changelog{
    pub fn all(conn: &MysqlConnection) -> Result<Option<Vec<Changelog>>, diesel::result::Error> {
        let cl = all_changelogs
            .order(changelog::time_gained.desc())
            .filter(changelog::time_gained.is_not_null())
            .filter(changelog::banned.eq(0))
            .limit(50)
            .load::<Changelog>(conn)?;
        // Wrapping the vector in a result and an option (not necessary but good practice)
        Ok(Some(cl))
    }
}
