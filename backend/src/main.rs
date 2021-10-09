#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use dotenv::dotenv;
use rayon::prelude::*;
use chrono::prelude::*;
use log::{info, trace, warn};

mod stages;
use stages::exporting::*;
use stages::fetching::*;

mod models;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Arg mapping
    
    // len == 1 (path)
    // Default that will check all SP/Coop Maps and update new scores and then re-compute points.
    // len == 3 (path + 1)
    // Option to determine other mode
    //      ssp -> Specific SP map, next arg is map#, will recompute points
    //      scp -> Specific Coop map, next arg is map#, will recompute points
    //      rcp -> Recompute points (useful after a ban/admin confirmation)
    
    // TODO: Stage point computation??
    // TODO: Handle caching of point information.
    
    dotenv().ok();
    if args.len() == 1{
        unimplemented!();
        //fetch_all()    
    }
    else if args.len() == 2{
        match args.get(1){
            Some(a) =>{
                match a.as_str(){
                    "rcp" => calc_points(vec![]),
                    _ => panic!("Incorrect value"),
                }
            }
            None => panic!("Incorrect Value"),
        }
    }
    else if args.len() == 3{
        match args.get(1){
            Some(a) =>{
                println!("{:?}", a);
                match a.as_str(){
                    "ssp" => fetch_sp(args.get(2).expect("Invalid map_id for arg #2").to_string()),
                    "scp" => fetch_cp(args.get(2).expect("Invalid map_id for arg #2").to_string()),
                    "rcp" => calc_points(vec![]),
                    _ => panic!("Incorrect value"),
                }
            },
            None => panic!("Incorrect value"),
        }
    }
    else {
        panic!("Incorrect arg #");
    }
}

fn fetch_all(){
    let official_sp = [47458,47455,47452,47106,47735,47736,47738,47742,
    47744,47465,47746,47748,47751,47752,47755,47756,47759,47760,47763,47764,47766,47768,47770,
    47773,47774,47776,47779,47780,47783,47784,47787,47468,47469,47472,47791,47793,47795,47798,
    47800,47802,47804,47806,47808,47811,47813,47815,47817,47819,47821,47824,47456];

    let official_coop = [47741,47825,47828,47829,45467,46362,47831,47833,
    47835,47837,47840,47841,47844,47845,47848,47849,47854,47856,47858,47861,52642,52660,52662,
    52663,52665,52667,52671,52687,52689,52691,52777,52694,52711,52714,52715,52717,52735,52738,
    52740,49341,49343,49345,49347,49349,49351,52757,52759,48287];

    let utc = Utc::now().naive_utc();
    let res_sp: Vec<_> = official_sp.into_par_iter().map(|map_id|{
        // TODO: Pass values like # of results as args to the binary
        fetch_entries(*map_id, 0, 450, utc, false)
    }).collect();
    let res_cp: Vec<_> = official_coop.into_par_iter().map(|map_id|{
        fetch_entries(*map_id, 0, 800, utc, true)
    }).collect();

    // What do we do with the leaderboards...
}

fn fetch_sp(map_id: String){
    let utc = Utc::now().naive_utc();
    let res_sp = fetch_entries(map_id.parse().expect("Error parsing map_id"), 0, 450, utc, false);
    // Recalculate the points on the given map. Force reset cache on webserver.
    // Setup an endpoint on the webserver to invalidate cache for a specific map.
    
}
fn fetch_cp(map_id: String){
    let utc = Utc::now().naive_utc();
    let res_coop = fetch_entries(map_id.parse().expect("Error parsing map_id"), 0, 800, utc, true);

}
fn calc_points(maps_altered: Vec<i32>){
    // NOTE: We could just recalculate points on a set of impacted chapters. We can reuse the cached values for unaffected chapters.
    // If a score update comes in for btg only, we only need to recalc aggtime/aggpoints in chapter 3. But we would still need to update all user profiles? This might save a small amount of time.
    // Additionally, we could also ignore players that do not have scores in that give chapter (very limited # of players, might not be worth the effort). 

    // par_iter hit endpoint for each chapter (1-6 coop, 7-15 sp)
    let all_maps_per_chapter: Vec<Vec<String>> =vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].into_par_iter().map(|chapter_id|{
        let url = format!("http://localhost:8080/api/chapters/{}", chapter_id);
        reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON")
        }).collect();
    

// Algorithm (TO BE IMPROVED)    
    // Scores: Pull all top 200 score data for current maps, break into different threads by chapter.
        // Chapter:
            // Create a hashmap, with key profile_number, value is a struct that contains data for all chapters (each chapter has a score and time).
            // NOTE: For concurrency, we might need to unsafe wrap, or do other shit to ensure that we can mutate the same struct instance accross multiple threads.
            // In theory, this should be okay, because each thread will only have mutable access to specific compontents of the struct.
        // Overall: 
            // SP
                // Sum all sp chapters.
            // Coop
                // Sum all coop chapters.
            // Overall
                // Sum both sp/coop.
// Cache     
    // Player Profile / Stats:
        // Stats    
            // # wrs
            // Points
            // Position
            // Avg placement
            // Best/worst
            // Newest/oldest
        // Scores
            // All score history (break this into smaller calls?), all aggregated time/points history.
    

}