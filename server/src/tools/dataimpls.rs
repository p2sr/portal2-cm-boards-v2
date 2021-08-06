#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error};

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use crate::tools::schema::{changelog, chapters, coopbundled, maps, scores, usersnew};
use crate::tools::schema::changelog::dsl::changelog as all_changelogs;
use crate::tools::schema::usersnew::dsl::usersnew as all_users;
use crate::tools::schema::maps::dsl::maps as all_maps;
use crate::tools::schema::coopbundled::dsl::coopbundled as all_coops;
use crate::tools::datamodels::*;
use crate::db::DbPool;

impl CoopPreviewPrelude{
    /// Only used internally by the `CoopPreview::show` method.
    /// Grabs the top 40 times on a Coop map. Used as a prelude because of limitations with diesel and aliases.
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
    /// Only used internally by the `CoopPreviews::show` method.
    /// Joins user information for a given set of scores for a map.
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
    /// Calls two internal functions to gather the necessary information, and truncates excess information.
    /// Uses manual filtering through hashmaps to eliminate duplicate times by players in accordance to how cooperative handles "carrying".
    /// The logic is slightly more complicated, and nearly identical to the logic in the handler for coop maps.
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



impl SpPreview{
    // TODO: Improve error handling.
    // TODO: Check on distinct_on support or MySQL.
    // WHY IS DISTINCT_ON ONLY SUPPORTED FOR POSTGRES?
    /// Grabs top 40 times in an SP map. This is a crutch to avoid filtering out too many times for duplicates from one user.
    /// We do this rather than using the `DISTINCT_ON` method in diesel & MySQL as it's not currently supported by Diesel, the hope is that this is changed in the future.
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

impl SpPreviews{
    /// Makes a call to grab all `map_ids`, grabs the map name off a call to `Map::get_name()` and
    /// calls`SpPreview::show()` for all 60 sp maps and filters out top 7 for distinct runners.
    // Top 40 to handle duplicates to generate top 7, that number should be safe right?
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

impl SpBanned{
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Vec<SpBanned>, diesel::result::Error>{
        let changelog_entries = all_changelogs
            .select((changelog::profile_number, changelog::score))
            .filter(changelog::banned.eq(1))
            .filter(changelog::map_id.eq(mapid))
            .order(changelog::score.asc())
            .load::<SpBanned>(conn);
        if let Ok(changelog_entries) = changelog_entries{
            return Ok(changelog_entries);
        } else{
            return Err(diesel::result::Error::NotFound);
        }
    }
}

impl CoopBanned{
    pub fn show(conn: &MysqlConnection, mapid: String) ->Result<Vec<CoopBanned>, diesel::result::Error>{
        let coopbundled_entries = all_coops
            .select((coopbundled::profile_number1, coopbundled::profile_number2, coopbundled::score))
            .filter(coopbundled::map_id.eq(mapid))
            .filter(coopbundled::banned.eq(1))
            .order(coopbundled::score.asc())
            .load::<CoopBanned>(conn);
        if let Ok(coopbundled_entries) = coopbundled_entries{
            return Ok(coopbundled_entries);
        } else{
            return Err(diesel::result::Error::NotFound);
        }
    }
}

impl Map{
    /// Grabs the map at a given steam_id
    pub fn show(conn: &MysqlConnection, id: String) -> Vec<Map> {
        all_maps
            .filter(maps::steam_id.eq(id))
            .load::<Map>(conn)
            .expect("Error Loading Maps")
    }
    /// Grabs all steam_ids for single player
    pub fn all_sp_mapids(conn: &MysqlConnection) -> Vec<String> {
        all_maps
            .select(maps::steam_id)
            .filter(maps::is_coop.eq(0))
            .load::<String>(conn)
            .expect("Error loading SP maps")
    }
    /// Grabs all steam_ids for Cooperative
    pub fn all_coop_mapids(conn: &MysqlConnection) -> Vec<String> {
        all_maps
            .select(maps::steam_id)
            .filter(maps::is_coop.eq(1))
            .load::<String>(conn)
            .expect("Error loading SP maps")
    }
    /// Grabs all map info and loads it into Map vectors
    pub fn all(conn: &MysqlConnection) -> Vec<Map> {
        all_maps
            .order(maps::id.desc())
            .load::<Map>(conn)
            .expect("Error loading all maps")
    }
    /// Grabs the map name for the map at the given steam_id
    pub fn get_name(conn: &MysqlConnection, mapid: String) -> Option<String>{
        all_maps
            .select((maps::name))
            .filter(maps::steam_id.eq(mapid))
            .first(conn)
            .expect("Cannot find mapname")
    }
}

impl Usersnew{
    /// Gets the boardname, steamname and avatar from the database for a specifc user.
    pub fn show(conn: &MysqlConnection, profilenum: String) -> Result<Option<UserMap>, diesel::result::Error>{
        let user = all_users
            .select((usersnew::boardname.nullable(), usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .find(profilenum)
            .first(conn)?;
        Ok(Some(user))
    }
    /// Search function to find if there's a player with a similar boardname in the database.
    pub fn check_board_name(conn: &MysqlConnection, nickname: String) -> Result<bool, diesel::result::Error>{
        let user = all_users
            .filter(usersnew::boardname.like(nickname))
            .filter(usersnew::banned.eq(0))
            .get_result::<Usersnew>(conn);
        // Dear god fix this error handling
        if let Ok(user) = user{
            return Ok(true);
        }
        else{
            Ok(false)
        }
    }
    pub fn showbanned(conn: &MysqlConnection) -> Result<Vec<String>, diesel::result::Error>{
        let user = all_users
            .select(usersnew::profile_number)
            .filter(usersnew::banned.eq(1))
            .load::<String>(conn)?;
        Ok(user)
    }
    pub fn check_banned(conn: &MysqlConnection, profilenum: String) -> Result<bool, diesel::result::Error>{
        let user = all_users
            .filter(usersnew::profile_number.eq(profilenum))
            .get_result::<Usersnew>(conn);
        if let Ok(user) = user{
            if user.banned == 1{
                return Ok(true)
            }
            else{
                Ok(false)
            }
        }
        else{
            return Err(diesel::result::Error::NotFound);
        }
    }
}

// TODO: Fix this when diesel adds support for aliased queries.
impl CoopMapPrelude{
    /// Work-around for lack of alias support, grabs all information except the second user's profile data for the times on a given map.
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

/// Returns a result wrapped option that wraps an instance of CoopMap
/// Makes calls to both CoopMapPrelude and individual Usernews.println!
/// Handles null partners with Option<None>'s.
// TODO: Look into non-blocking, concurrent alternatives while we're using this work-around.
impl CoopMap{
    /// Calls `CoopMapPrelude::show` and `Usersnew::show` to fill in data due to lack of alias support in diesel. Returns all coop map information for a given map.actix_web
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
            // TODO: FIXME: Awful Error Handling
            let vec_final = Vec::new();
            Ok(Some(vec_final))
        }
    }
}

// TODO: Support for `DISTINCT_ON` in the future.
impl SpMap{
    /// Selects the necessary information for the sp maps page, filters out banned times and users
    pub fn show(conn: &MysqlConnection, mapid: String) -> Result<Option<Vec<SpMap>>, diesel::result::Error>{
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
            .load::<SpMap>(conn)?;
        // Wrapping the vector in a result and an option (not necessary but good practice)
        Ok(Some(map))
    }
}

impl Changelog{
    // TODO: More specific error handling (only return OK when "not found" is the error.)
    pub fn check_banned_scores(conn: &MysqlConnection, mapid: String, score: i32, profilenumber: String) -> Result<bool, diesel::result::Error>{
        let res = all_changelogs
            .filter(changelog::score.eq(score))
            .filter(changelog::map_id.eq(mapid))
            .filter(changelog::profile_number.eq(profilenumber))
            .filter(changelog::banned.eq(1))
            .load::<Changelog>(conn);
        match res{
            Ok(_) => return Ok(true),
            Err(e) => return Ok(false),
        }
    }
    // TODO: Better error handling.
    pub fn sp_pb_history(conn: &MysqlConnection, mapid: String, profilenumber: String) -> Result<Option<Vec<Changelog>>, diesel::result::Error>{
        let res = all_changelogs
            .filter(changelog::profile_number.eq(profilenumber))
            .filter(changelog::map_id.eq(mapid))
            .order(changelog::score.asc())
            .load::<Changelog>(conn);
        if let Ok(res) = res{
            return Ok(Some(res));
        } else{
            Ok(None)
        }
    }
}

impl ChangelogPage{
    /// Returns `limit` number of changelog entries. Handle the joining for map and user data.
    pub fn show(conn: &MysqlConnection, limit: i32) -> Result<Option<Vec<ChangelogPage>>, diesel::result::Error>{ 
        let cl = all_changelogs
            .inner_join(all_users)
            .inner_join(all_maps.on(changelog::map_id.eq(maps::steam_id)))
            .select((changelog::time_gained.nullable(), changelog::profile_number,
            changelog::score, changelog::map_id, changelog::wr_gain, 
            changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
            changelog::previous_id.nullable(), changelog::id, changelog::coopid.nullable(),
            changelog::post_rank.nullable(), changelog::pre_rank.nullable(),
            changelog::submission, changelog::note.nullable(), 
            changelog::category.nullable(), maps::name, usersnew::boardname.nullable(), 
            usersnew::steamname.nullable(), usersnew::avatar.nullable()))
            .order(changelog::time_gained.desc())
            //.filter(changelog::time_gained.is_not_null())
            .filter(usersnew::banned.eq(0))
            .filter(changelog::profile_number.ne("".to_string()))
            .limit(limit.into())
            .load::<ChangelogPage>(conn)?;
        Ok(Some(cl))
    }
    // TODO: Make this a struct dear god...
    /// Filtering options for the changelog through any of the options passed. For more information, checkout `ChangelogQueryParams` in `datamodels.rs`
    pub fn show_filtered(conn: &MysqlConnection, nickname: Option<String>, 
        profilenumber: Option<String>, chamber: Option<String>,  sp: Option<i32>, 
        coop: Option<i32>, wrgain: Option<i32>, hasdemo: Option<i32>, hasvideo: Option<i32>,
        limit: i32) ->  Result<Option<Vec<ChangelogPage>>, diesel::result::Error>
        {
        let mut query = all_changelogs
        .inner_join(all_users)
        .inner_join(all_maps.on(changelog::map_id.eq(maps::steam_id)))
        .select((changelog::time_gained.nullable(), changelog::profile_number,
        changelog::score, changelog::map_id, changelog::wr_gain, 
        changelog::has_demo.nullable(), changelog::youtube_id.nullable(), 
        changelog::previous_id.nullable(), changelog::id, changelog::coopid.nullable(),
        changelog::post_rank.nullable(), changelog::pre_rank.nullable(),
        changelog::submission, changelog::note.nullable(), 
        changelog::category.nullable(), maps::name, usersnew::boardname.nullable(), 
        usersnew::steamname.nullable(), usersnew::avatar.nullable()))
        .order(changelog::time_gained.desc())
        .filter(usersnew::banned.eq(0))
        .into_boxed();
        if let Some(sp) = sp{
            query = query.filter(maps::is_coop.ne(sp));
        }
        else if let Some(coop) = coop{
            query = query.filter(maps::is_coop.eq(coop));
        }
        if let Some(hasdemo) = hasdemo{
            query = query.filter(changelog::has_demo.eq(1));
        }
        if let Some(hasvideo) = hasvideo{
            query = query.filter(changelog::youtube_id.is_not_null());
        }
        if let Some(wrgain) = wrgain{
            query = query.filter(changelog::wr_gain.eq(1));
        }
        if let Some(chamber) = chamber{
            query = query.filter(changelog::map_id.eq(chamber));
        }
        if let Some(profilenumber) = profilenumber{
            query = query.filter(changelog::profile_number.eq(profilenumber));
        }
        #[allow(irrefutable_let_patterns)]
        if let Some(nickname) = nickname{
            if let namecheck = Usersnew::check_board_name(&conn, nickname.clone()){
                query = query.filter(usersnew::boardname.eq(nickname.clone()));
            }else{
                query = query.filter(usersnew::steamname.eq(nickname));
            }
        }
        let result = query.limit(limit.into())
            .load::<ChangelogPage>(conn)?;
        Ok(Some(result))
    }
}