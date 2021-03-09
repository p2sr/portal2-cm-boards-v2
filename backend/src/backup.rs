#![allow(dead_code)]
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::schema::changelog;
use crate::schema::changelog::dsl::changelog as all_changelogs;
use crate::schema::maps;
use crate::schema::maps::dsl::maps as all_maps;

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
        handle pulling of Changelog information
        from the database. It exists to be able to save MySQL data in Rust. */

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

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Maps {
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: Option<String>,
    pub type_: String,
    pub chapter_id: Option<u32>,
    pub is_coop: i32,
    pub is_public: i32,
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
        use crate::schema::changelog::dsl::
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
impl Maps{
    pub fn show(id: String, conn: &MysqlConnection) -> Vec<Maps> {
        all_maps
            .filter(maps::steam_id.eq(id))
            .load::<Maps>(conn)
            .expect("Error Loading Maps")
    }
    pub fn all(conn: &MysqlConnection) -> Vec<Maps> {
        all_maps
            .order(maps::id.desc())
            .load::<Maps>(conn)
            .expect("Error loading all maps")
    }
    /*  We shouldn't need any update methods for maps. 
        Map changes should probably not be handled through the boards.*/
    
}