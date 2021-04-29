#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::all)]
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::models::schema::changelog;
use crate::models::schema::changelog::dsl::changelog as all_changelogs;
use crate::models::schema::maps;
use crate::models::schema::maps::dsl::maps as all_maps;

use crate::models::schema::chapters;
use crate::models::schema::coopbundled;
use crate::models::schema::coopbundled::dsl::coopbundled as all_coopbundled;
use crate::models::schema::scores;
use crate::models::schema::usersnew;
use crate::models::schema::usersnew::dsl::usersnew as all_users;

/*NOTES:
    The Option<> wrapper is used to handle NULL values, NULL is not present
    in rust, therefore Option<> is a built-in enum wrapper for to handle
    NULL in Rust. Any NULLABLE column should be Option<> wrapped.
    Derives for structs below: 
    Serialize allows seralization of this struct
    Queryable allows for us to query based off this structure
    Debug allows for debug output
    Clone allows us to clone the struct*/

/*`pub struct Changelog`
    Handle pulling of Changelog information from the database. It exists to be
    able to save MySQL data in Rust. */

#[derive(Serialize, Queryable, Debug, Clone)]
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
#[derive(Queryable, Debug, Identifiable)]
pub struct Chapter {
    pub id: u32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: i32,
}
#[derive(Serialize, Queryable, Debug, Clone)]
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
#[derive(Queryable, Debug, Identifiable)]
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
#[derive(Queryable, Debug, Identifiable)]
#[primary_key(changelog_id)]
pub struct Score {
    pub profile_number: String,
    pub map_id: String,
    pub changelog_id: i32,
}
#[derive(Queryable, Debug, Serialize, Clone)]
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
/*`pub struct SpMapPage`
    Allows for the joining of the changelog and usersnew tables to be able
    to bundle both in a unified query.*/
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct SpMapPage {
    pub score_data: Changelog,
    pub user_data: Usersnew,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMapPagePrelude {
    pub score_data: Coopbundled,
    pub user1_data: Usersnew,
}

/*`pub struct CoopMapPage`
    Allows for the joining of the coopbundled and usersnew tables to be able
    to bundle both in a unified query. NOT CURRENTLY SUPPORTED IN DIESEL*/
#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CoopMapPage {
    pub score_data: Coopbundled,
    pub user1_data: Usersnew,
    pub user2_data: Usersnew,
}
/*Derives for the structs below:
    Deserialize allows us to read in from JSON
    Insertable allows us to insert into MySQL
    `table_name` references the corresponding !table in schema.rs*/

/*`pub struct NewChangelog`:i32
    exists to allow us to construct a new changelog entity to insert into 
    our database. The ID is not present, as it is auto-incremented in the db.*/

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "changelog"]
pub struct NewChangelog{
    pub time_gained: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>,
    pub banned: i32,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub coopid: Option<i32>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: i32,
    pub note: Option<String>,
    pub category: Option<String>,
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
// Reused server code
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize)]
pub struct SPRanked{
    pub map_data: SPMap,
    pub rank: i32,
    pub score: f32,
}

impl Changelog{
    pub fn show(id: i32, conn: &MysqlConnection) -> Vec<Changelog> {
        all_changelogs
            .find(id)
            .load::<Changelog>(conn)
            .expect("Error Loading Changelog")
    }
    pub fn all(conn: &MysqlConnection) -> Vec<Changelog> {
        all_changelogs
            .order(changelog::id.desc())
            .load::<Changelog>(conn)
            .expect("Error loading all changelog")
    }
    /*  I think having this handle updating the entire struct makes more senese.
        This way, we grab the existing struct before updating, change the struct how we want,
        Then re-pass the entire struct to update (all but ID).*/
    pub fn update_by_id(id: i32, conn: &MysqlConnection, changelog: NewChangelog) -> bool{
        use crate::models::schema::changelog::dsl::
            {time_gained as tg,
            profile_number as pn,
            score as s,
            map_id as mid,
            wr_gain as wr,
            has_demo as hd,
            banned as b,
            youtube_id as yid,
            previous_id as pid,
            coopid as cid,
            post_rank as por,
            pre_rank as prr,
            submission as sb,
            note as n,
            category as c};
        
        let NewChangelog{
            time_gained,
            profile_number,
            score,
            map_id,
            wr_gain,
            has_demo,
            banned,
            youtube_id,
            previous_id,
            coopid,
            post_rank,
            pre_rank,
            submission,
            note,
            category,
        } = changelog;
        
        diesel::update(all_changelogs.find(id))
            .set((tg.eq(time_gained), 
            pn.eq(profile_number),
            s.eq(score),
            mid.eq(map_id),
            wr.eq(wr_gain),
            hd.eq(has_demo),
            b.eq(banned),
            yid.eq(youtube_id),
            pid.eq(previous_id),
            cid.eq(coopid),
            por.eq(post_rank),
            prr.eq(pre_rank),
            sb.eq(submission),
            n.eq(note),
            c.eq(category)))
            .execute(conn)
            .is_ok()
    }
    pub fn insert(changelog: NewChangelog, conn: &MysqlConnection) -> bool{
        diesel::insert_into(changelog::table)
            .values(&changelog)
            .execute(conn)
            .is_ok()
    }
    // Do we need a deletion function??? Probably not, but for the sake of testing
    pub fn delete_by_id(id: i32, conn: &MysqlConnection) -> bool{
        if Changelog::show(id, conn).is_empty(){
            return false;
        };
        diesel::delete(all_changelogs.find(id)).execute(conn).is_ok()
    }
    pub fn all_by_profile_num(pnum: String, conn: &MysqlConnection) -> Vec<Changelog>{
        all_changelogs 
            .filter(changelog::profile_number.eq(pnum))
            .load::<Changelog>(conn)
            .expect("Error loading changelog by profile number")
    }
    // DEPRECIATED: This function mostly just exists as a test, might re-implement for filtering in the future.
    pub fn all_by_map_id(mapid: String, conn: &MysqlConnection) -> Vec<Changelog>{
        all_changelogs
        .filter(changelog::map_id.eq(mapid))
        .filter(changelog::banned.eq(0))
        .limit(200)
        .order(changelog::score.asc())
        .load::<Changelog>(conn)
        .expect("Error loading all changelog")       
    }
}

impl Map{
    pub fn show(id: String, conn: &MysqlConnection) -> Vec<Map> {
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
    /*  We shouldn't need any update methods for maps. 
        Map changes should probably not be handled through the boards.*/
}

impl SpMapPage{
    pub fn show(mapid: String, conn: &MysqlConnection) -> Vec<SpMapPage> {
        all_changelogs
            .inner_join(all_users)
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(changelog::score.asc())
            .load::<SpMapPage>(conn)
            .expect("Error loading all map pages.")
    }
}

impl Coopbundled{
    pub fn show(mapid: String, conn: &MysqlConnection) -> Vec<Coopbundled>{
        all_coopbundled
            .filter(coopbundled::map_id.eq(mapid))
            .filter(coopbundled::banned.eq(0))
            .order(coopbundled::score.asc())
            .load::<Coopbundled>(conn)
            .expect("Error loading coopbundled")
    }
}

impl Usersnew{
    pub fn show(userid: String, conn: &MysqlConnection) -> Usersnew {
        all_users
            .find(userid)
            .first(conn)
            .expect("Error loading user.")
    }
}

impl CoopMapPagePrelude{
    pub fn show(mapid: String, conn: &MysqlConnection) -> Vec<CoopMapPagePrelude>{
        all_coopbundled
            .inner_join(all_users)
            .filter(coopbundled::map_id.eq(mapid))
            .filter(coopbundled::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(coopbundled::score.asc())
            .load::<CoopMapPagePrelude>(conn)
            .expect("Error loading coopbundled prelude")
    }
}

impl CoopMapPage{
    pub fn show(mapid: String, conn: &MysqlConnection) -> Vec<CoopMapPage> {
        let nouser = Usersnew {profile_number: "".to_string(), boardname: None, steamname: None, banned: 0, registered: 0, avatar: None, twitch: None, youtube: None, title: None, admin: 0, donation_amount: None};
        let mut vecfinal = Vec::new();
        let cb = CoopMapPagePrelude::show(mapid, &conn);
        for pre in cb.iter(){
            let tempstr = &pre.score_data.profile_number2;
            if(tempstr != ""){
                let user_two = Usersnew::show(tempstr.to_string(), &conn);
                let tempstruct = CoopMapPage {score_data: pre.score_data.clone(), user1_data: pre.user1_data.clone(), user2_data: user_two.clone()}; 
                vecfinal.push(tempstruct);
            }
            else{
                let tempstruct = CoopMapPage {score_data: pre.score_data.clone(), user1_data: pre.user1_data.clone(), user2_data: nouser.clone()};
                vecfinal.push(tempstruct);
            }
        }
        return vecfinal;
    }
}

// Need support for aliased queries. https://github.com/diesel-rs/diesel/pull/2254
/*
impl CoopMapPage{
    pub fn show(mapid: String, conn: &MysqlConnection) -> Vec<CoopMapPage> {
        all_changelogs
            .inner_join(usersnew::table.on(usersnew::profile_number.eq(coopbundled::profile_number1)))
            .inner_join(usersnew::table.on(usersnew::profile_number.eq(coopbundled::profile_number2)))
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::banned.eq(0))
            .filter(usersnew::banned.eq(0))
            .order(changelog::score.asc())
            .load::<CoopMapPage>(conn)
            .expect("Error loading all map pages.")
    }
}
*/