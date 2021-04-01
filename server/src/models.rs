#![allow(unused)]
#![allow(clippy::all)]

use actix_web::{HttpResponse, Error};

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::schema::{changelog, chapters, coopbundled, maps, scores, usersnew};
use crate::schema::changelog::dsl::changelog as all_changelogs;
use crate::schema::usersnew::dsl::usersnew as all_users;
use crate::schema::coopbundled::dsl::coopbundled as all_coops;

use crate::db::DbPool;
// Structs are generated off the database (using deisel_ext) and modified to be used to store query data.
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


// This struct handles the minimal information we want for SP map pages. We want to limit the amount of data we need to transfer.
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

// This struct is a work-around for the issues with aliased queries in diesel, ideally this would be scrapped for an aliased join on usersnew
// so we could grab both sets of usersnew information in one query
// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
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

// This is the big brother struct for the prelude to handle all of the data. The overhead on copy all the data is relatively small, 
// but ideally we would only need this and not the prelude.
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

// Grabs just the essential user information to aid in filling in the CoopMapPrelude
// TODO: Potentially work boardname and steamname into one field? (Check if boardname exists, if it doesn, keep it, if not, replace it with steamname)
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UserMap{
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

// impl block for Usersnew
    // methods:
        // show(conn: &MysqlConnection, profile_number: String)
            // Returns a result wrapped option that wraps an instance of UserMap (I know, redundant...)
impl Usersnew{
    pub fn show(conn: &MysqlConnection, profilenum: String) -> Result<Option<UserMap>, diesel::result::Error>{
        let user = all_users
            .select((usersnew::boardname.nullable(), usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .find(profilenum)
            .first(conn)?;
        Ok(Some(user))
    }
}

// impl block for CoopMapPrelude
    // methods:
        // show(conn: &MysqlConnection, map_id: String)
            // Returns a result wrapped option that wraps an instance of CoopMapPrelude, used to grab the initial join for coop map pages
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

// impl block for CoopMap
    // methods:
        // show(conn: &MysqlConnection, map_id: String) 
            // Returns a result wrapped option that wraps an instance of CoopMap
            // Makes calls to both CoopMapPrelude and individual Usernews.println!
            // Handles null partners with Option<None>'s.
// TODO: Look into non-blocking, concurrent alternatives while we're using this work-around.
impl CoopMap{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<CoopMap>>, diesel::result::Error>{
        let coop_prelude = CoopMapPrelude::show(&conn, mapid.clone())?;
        if let Some(coop_prelude) = coop_prelude {
            let mut vecfinal = Vec::new();
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
                        vecfinal.push(tempstruct);
                    } else{
                        println!("Unexpected Error");
                    }
                }
                else{
                    let tempstruct = CoopMap {time_gained: entry.time_gained, profile_number1: entry.profile_number1, profile_number2: entry.profile_number2, score: entry.score, is_blue: entry.is_blue,
                        has_demo1: entry.has_demo1, has_demo2: entry.has_demo2, youtube_id1: entry.youtube_id1, youtube_id2: entry.youtube_id2, submission1: entry.submission1, submission2: entry.submission2,
                        note1: entry.note1, note2: entry.note2, category: entry.category, boardname1: entry.boardname, steamname1: entry.steamname, avatar1: entry.avatar, boardname2: None, 
                        steamname2: None, avatar2: None};
                    vecfinal.push(tempstruct);
                }
            }
            Ok(Some(vecfinal))
        } else{
            //FIXME: Awful Error Handling
            let vecfinal = Vec::new();
            Ok(Some(vecfinal))
        }
        
    }
}
// impl block for SPMap
    // methods:
        // show(conn: &MysqlConnection, map_id: String) 
            // Returns a result wrapped option that wraps an instance of SPMap
            // Selects the necessary information for the maps page, filters out banned times and users
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
