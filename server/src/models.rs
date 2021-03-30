#![allow(unused)]
#![allow(clippy::all)]

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::schema::{changelog, chapters, coopbundled, maps, scores, usersnew};
use crate::schema::changelog::dsl::changelog as all_changelogs;
use crate::schema::usersnew::dsl::usersnew as all_users;

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

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name = "changelog"]
pub struct MapChangeLog{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub has_demo: Option<i32>,
    pub youtube_id: Option<String>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>, 
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name = "usersnew"]
pub struct MapUsersnew{
    pub boardname: Option<String>,
    pub steamname: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct ShortSPMap{
    pub score_data: MapChangeLog,
    pub user_data: MapUsersnew,
}

impl ShortSPMap{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<ShortSPMap>>, diesel::result::Error>{
        let map = all_changelogs            
            .inner_join(all_users)
            .select((changelog::time_gained.nullable(), changelog::profile_number, changelog::score, 
            changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
            changelog::submission, changelog::note.nullable(), 
            changelog::category.nullable(), usersnew::boardname.nullable(), usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(changelog::score.asc())
            .load::<ShortSPMap>(conn)?;
        Ok(Some(map))
    }
}
/*
let names_and_titles = join.select((users::name, posts::title.nullable()))
    .load::<(String, Option<String>)>(&connection)?;
let expected_data = vec![
    (String::from("Sean"), Some(String::from("Sean's Post"))),
    (String::from("Tess"), None),
];
assert_eq!(expected_data, names_and_titles);
*/
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
