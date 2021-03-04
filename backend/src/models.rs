use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
//use diesel::sql_types::Timestamp;
use chrono::NaiveDateTime;
//use diesel::deserialize::FromSql;

// http://diesel.rs/guides/schema-in-depth/
use crate::schema::changelog;
use crate::schema::changelog::dsl::changelog as all_changelogs;
//use crate::schema::dsl::*;
use chrono::{DateTime, Utc};

// https://github.com/serde-rs/serde/issues/1151
// https://serde.rs/custom-date-format.html 
// Implementing custom date format
mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Changelog {
    //pub time_gained: Option<Timestamp>,
    #[serde(with = "my_date_format")]
    // TODO: Fix date struct
    pub time_gained: DateTime<Utc>, // NULLABLE
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>, // NULLABLE
    pub banned: i32,
    pub youtube_id: Option<String>, // NULLABLE
    pub previous_id: Option<i32>, // NULLABLE
    pub id: i32,
    pub coopid: Option<i32>, // NULLABLE
    pub post_rank: Option<i32>, // NULLABLE
    pub pre_rank: Option<i32>, // NULLABLE
    pub submission: i32,
    pub note: Option<String>, // NULLABLE
    pub category: Option<String>, // NULLABLE
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "changelog"]
pub struct NewChangelog{
    // TODO: Fix date struct
    pub time_gained: DateTime<Utc>,// NULLABLE
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub wr_gain: i32,
    pub has_demo: Option<i32>, // NULLABLE
    pub banned: i32,
    pub youtube_id: Option<String>, // NULLABLE
    pub previous_id: Option<i32>, // NULLABLE
    pub coopid: Option<i32>, // NULLABLE
    pub post_rank: Option<i32>, // NULLABLE
    pub pre_rank: Option<i32>, // NULLABLE
    pub submission: i32,
    pub note: Option<String>, // NULLABLE
    pub category: Option<String>, // NULLABLE
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
    // I think having this handle updating the entire struct makes more senese.
    // This way, we grab the existing struct before updating, change the struct how we want,
    // Then re-pass the entire struct to update (all but ID).
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
    // Do we need a deletion function???
    // Probably not, but for the sake of testing
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
}
