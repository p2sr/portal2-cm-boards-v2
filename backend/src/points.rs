use rayon::prelude::*;
use std::collections::HashMap;
use crate::models::datamodels::{SpMap, SpRanked, CoopMap, CoopRanked};

pub fn calc_points(maps_altered: Vec<i32>){
    // NOTE: We could just recalculate points on a set of impacted chapters. We can reuse the cached values for unaffected chapters.
    // If a score update comes in for btg only, we only need to recalc aggtime/aggpoints in chapter 3. But we would still need to update all user profiles? This might save a small amount of time.
    // Additionally, we could also ignore players that do not have scores in that give chapter (very limited # of players, might not be worth the effort). 

    // par_iter hit endpoint for each chapter (1-6 coop, 7-15 sp)
    let all_maps_per_chapter: Vec<Vec<String>> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].into_par_iter().map(|chapter_id|{
        let url = format!("http://localhost:8080/api/chapters/{}", &chapter_id);
        let map_ids: Vec<String> = reqwest::blocking::get(&url)
            .expect("Error in query to our local API (Make sure the webserver is running")
            .json()
            .expect("Error in converting our API values to JSON");
        calc_chapter(map_ids, chapter_id);
    }).collect();
}

pub fn calc_chapter(map_ids: Vec<String>, chapter_id: i32){
    let final_hm: HashMap<String, (f32, i32)> = HashMap::with_capacity(1050);
    for map in map_ids.iter(){
        // Grab top 200 from the web-server for each map.
        if chapter_id > 6 { //SP
            let res: Vec<SpRanked> = reqwest::blocking::get(format!("http://localhost:8080/api/maps/sp/{}", &map))
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            for score in res{ // If an entry exists, add this new value to the old value, if not, keep only new value.
                match final_hm.insert(score.map_data.profile_number, (score.points, score.map_data.score)){
                    Some((old_points, old_score)) => {final_hm.insert(score.map_data.profile_number, (score.points + old_points, score.map_data.score + old_score));},
                    None => (),
                }
            }
        } else{ // Coop
            // TODO: Check for a player already having a better time on a given map, do not double up points.
            let res: Vec<CoopMap> = reqwest::blocking::get(format!("http://localhost:8080/api/maps/coop/{}", &map))
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
        }
    }
    // Have this be a map, map all the values into one big hashmap.
    // final_hm = map_ids.into_par_iter().map(|map: String|{
    //     // Grab top 200 from the web-server for each map.
    //     if chapter_id > 6 { //SP
    //         let res: Vec<SpRanked> = reqwest::blocking::get(format!("http://localhost:8080/api/maps/sp/{}", &map))
    //             .expect("Error in query to our local API (Make sure the webserver is running")
    //             .json()
    //             .expect("Error in converting our API values to JSON");
    //         for score in res.iter(){

    //         }
    //     } else{ // Coop
    //         let res: Vec<CoopMap> = reqwest::blocking::get(format!("http://localhost:8080/api/maps/coop/{}", &map))
    //             .expect("Error in query to our local API (Make sure the webserver is running")
    //             .json()
    //             .expect("Error in converting our API values to JSON");
    //     }
    // }).collect();
}

// trait HasPoints{
//     fn get_points(&self) -> f32;
//     fn get_time(&self) -> i32;
//     fn get_profile_number(&self) -> (String, Option<String>);
// }

// impl HasPoints for SpRanked{
//     fn get_points(&self) -> f32{
//         self.points
//     }
//     fn get_time(&self) -> i32{
//         self.map_data.score
//     }
//     fn get_profile_number(&self) -> (String, Option<String>){
//         (self.map_data.profile_number, None)
//     }
// }

// impl HasPoints for CoopRanked{
//     fn get_points(&self) -> f32{
//         self.points
//     }
//     fn get_time(&self) -> i32{
//         self.map_data.score
//     }
//     fn get_profile_number(&self) -> (String, Option<String>){
//         (self.map_data.profile_number1, Some(self.map_data.profile_number2))
//     }
// }

// pub fn extract_points<T>(page: Vec<T>) 
//     where T: HasPoints,
// {

// }

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
