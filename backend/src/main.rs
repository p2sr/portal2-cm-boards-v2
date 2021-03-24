#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

mod schema;
mod models;
use models::SpMapPage;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set the DATABASE_URL in ../.env");
    let conn = MysqlConnection::establish(&database_url).unwrap();
    println!("Caching all SP maps");
    output_sp_maps(&conn);
    println!("Caching all Coop maps");
    output_coop_maps(&conn);
}

fn output_sp_maps(conn: &MysqlConnection){
    let all_sp_maps = models::Map::all_sp_mapids(&conn);
    
    /* This loop goes through all the SP maps, grabs all their changelog entities 
       in order from lowest score (filters banned times), filters worse times from
       a player (outdated times), and outputs the changelog and player data as json*/

    /* TODO: Filter out specific information for the map prints (we don't need all 
       changelog and all user data included, just specific parts.), implement a variable
       in the database to track if a time is outdated to reduce compute time in Rust*/
  
    for map_id in all_sp_maps.iter(){
        let map_str = format!("../server/api/maps/sp/{}.json", map_id.to_string());
        let path = Path::new(&map_str);
        
        // We get a vector of all the non-banned times on a given map, as a bundled changelog and usersnew
        let changelog_entries = models::SpMapPage::show(map_id.to_string(), &conn);
        
        // Filter out runners with multiple times
        let mut changelog_entries_filtered = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(200);
        for entry in changelog_entries.iter(){
            match remove_dups.insert(entry.score_data.profile_number.clone(), 1){
                // If this returns, the profile_number has a better time, remove the time from the vector
                Some(_) => (),
                _ => changelog_entries_filtered.push(entry.clone()),
            }
        }
        let file = File::create(path).unwrap();
        // Limits to 200 results.
        changelog_entries_filtered.truncate(200);
        serde_json::to_writer_pretty(file, &changelog_entries_filtered).unwrap();
    }
}
// Will need to be worked on, aliased queries not currently supported native by diesel.
#[allow(unused_variables)]
fn output_coop_maps(conn: &MysqlConnection){
    let all_coop_maps = models::Map::all_coop_mapids(&conn);

    for map_id in all_coop_maps.iter(){
        let map_str = format!("../server/api/maps/coop/{}.json", map_id.to_string());
        let path = Path::new(&map_str);

        let coopbundled_entries = models::CoopMapPage::show(map_id.to_string(), &conn);   
        
        let mut coopbundled_entries_filtered = Vec::new();
        let mut remove_dups: HashMap<String, i32> = HashMap::with_capacity(500);
        remove_dups.insert("".to_string(), 1);
        for entry in coopbundled_entries{
            match remove_dups.insert(entry.score_data.profile_number1.clone(), 1){
                // If player 1 has a better time, check to see if player 2 doesn't.
                Some(_) => match remove_dups.insert(entry.score_data.profile_number2.clone(), 1){
                    Some(_) => (),
                    _ => coopbundled_entries_filtered.push(entry.clone()),
                }
                // This case handles if player 1 doesn't have a better time, and it tries to add player 2 in as well, if two has a better time or not, this is included.
                _ => match remove_dups.insert(entry.score_data.profile_number2.clone(), 1){
                    Some(_) => coopbundled_entries_filtered.push(entry.clone()),
                    _ => coopbundled_entries_filtered.push(entry.clone()),
                }
            }
        }
        // Quick test to see output in a more visual and friendly way
        // if map_id == "47741"{ 
        //     for entry in coopbundled_entries_filtered.iter(){
        //         println!("Score - {:?} \tPlayer 1 - {:?} \t\tPlayer 2 - {:?}", entry.score_data.score, entry.user1_data.steamname, entry.user2_data.steamname);
        //     }
        // }
        let file = File::create(path).unwrap();
        // Limits to 200 results.
        coopbundled_entries_filtered.truncate(200);
        serde_json::to_writer_pretty(file, &coopbundled_entries_filtered).unwrap();
    }
}