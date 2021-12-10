use rayon::prelude::*;
use std::collections::HashMap;
use crate::models::datamodels::{SpMap, SpRanked, CoopMap, CoopRanked};

#[derive(Debug, Clone)]
pub struct Points{
    points: f32,
    score: i32,
    num_scores: i32,
    total_rank_sum: i32,
    worst: (i32, String), 
    best: (i32, String),
}

impl Points{
    pub fn sum(self, new_points: &Points) -> Points{
        Points{
            points: self.points + new_points.points,
            score: self.score + new_points.score,
            num_scores: self.num_scores + new_points.num_scores,
            total_rank_sum: self.total_rank_sum + new_points.total_rank_sum,
            worst: if new_points.worst.0 > self.worst.0 { // TODO: When there are multiple that are equal, have a state for "multiple times"
                (new_points.worst.0, new_points.worst.1.clone())
            } else{
                self.worst
            },
            best: if new_points.best.0 < self.best.0{
                (new_points.best.0, new_points.best.1.clone())
            }else {
                self.best
            },
        }
    }
}

// NOTE: Find a better way to do this...
pub fn score_calc(score: i32) -> (i32, i32, i32) {
    let score_str = score.to_string();
    let (seconds, ms) = score_str.split_at(score_str.len()-2);
    let mut seconds = seconds.parse::<i32>().unwrap();
    let ms = ms.parse::<i32>().unwrap();
    let mut minutes = 0;
    while seconds >= 60{
        minutes += 1;
        seconds -= 60;
    }
    (minutes, seconds, ms)
}

pub fn calc_points(maps_altered: Option<Vec<i32>>) {
    // NOTE: We could just recalculate points on a set of impacted chapters. We can reuse the cached values for unaffected chapters.
    // If a score update comes in for btg only, we only need to recalc aggtime/aggpoints in chapter 3. But we would still need to update all user profiles? This might save a small amount of time.
    // Additionally, we could also ignore players that do not have scores in that give chapter (very limited # of players, might not be worth the effort). 
    let mut overall_vec : HashMap<String, Points> = HashMap::with_capacity(50 * 200);
    if maps_altered == None {
        // Contains a vector of tuples, each hashmap stores the total points for each player, per chapter. Chapters are denoted by the i32 in the tuple.
        // NOTE: The par_iter means we could have the chapters finish calculating in any order, and thus the ordering can not be assumed.
        let mut hm_vec = Vec::with_capacity(15);
        // par_iter hit endpoint for each chapter (1-6 coop, 7-15 sp)
        hm_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].into_par_iter().map(|chapter_id|{
            let url = format!("http://localhost:8080/api/chapters/{}", &chapter_id);
            let map_ids: Vec<String> = reqwest::blocking::get(&url)
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            calc_chapter(map_ids, chapter_id)
        }).collect();
        // TODO: Send the points over to the webserver for the webserver to cache before consuming the vector for the overall calculations.

        // Generate aggregated overall.
        for (chapter, chapter_hm) in hm_vec {
            for (profile_number, new_points) in chapter_hm {
                // Might need to switch this pattern.
                match overall_vec.get(&profile_number){
                    Some(old_points) => {
                        match overall_vec.insert(profile_number.clone(), new_points.sum(old_points)){ //old_points.sum(&new_points)){
                            Some(_) => (),
                            None => unreachable!(),
                        }
                    },
                    None => {
                        match overall_vec.insert(profile_number.clone(), new_points.clone()){ // TODO: Maybe remove clone?
                            Some(_) => unreachable!(),
                            None => (),
                        }
                    },
                }

            }
        }
        println!("{:#?}", overall_vec.get("76561198039230536"));
    } else {
        ()
    }
}

/// Function to go chapter-by-chapter to calculate points.
pub fn calc_chapter(map_ids: Vec<String>, chapter_id: i32) -> (i32, HashMap<String, Points>) {
    // Keep track of total points/score (time) per chapter
    let mut final_hm: HashMap<String, Points> = HashMap::with_capacity(10 * 200);
    // Make sure to track which players have a score already on a given map (coop)
    let mut coop_hm: HashMap<String, i32> = HashMap::with_capacity(200);
    for map in map_ids.iter() {
        // Grab top X from the web-server for each map.
        if chapter_id > 6 { // SP
            let url = format!("http://localhost:8080/api/maps/sp/{}", &map).to_string();
            let res: Vec<SpRanked> = reqwest::blocking::get(&url) // Assumes all top 200
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            for score in res { // If an entry exists, add this new value to the old value, if not, keep only new value.
                match final_hm.insert(score.map_data.profile_number.clone(), Points{
                    points: score.points,
                    score: score.map_data.score,
                    num_scores: 1,
                    total_rank_sum: score.rank,
                    worst: (score.rank, map.clone()),
                    best: (score.rank, map.clone()),
                }){  
                    Some(pnts) => {final_hm.insert(score.map_data.profile_number, Points{
                        points: pnts.points + score.points,
                        score: pnts.score + score.map_data.score,
                        num_scores: pnts.num_scores + 1,
                        total_rank_sum: pnts.total_rank_sum + score.rank,
                        worst: if score.rank > pnts.worst.0 {
                            (score.rank, map.clone())
                        } else{
                            pnts.worst
                        },
                        best: if score.rank < pnts.best.0{
                            (score.rank, map.clone())
                        }else {
                            pnts.best
                        },
                    });},
                    None => (),
                }
            }
        } else{ // Coop
            let url = format!("http://localhost:8080/api/maps/coop/{}", &map);
            let res: Vec<CoopRanked> = reqwest::blocking::get(&url) // Assumes all top 200
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            for score in res {
                // Do checks for both player 1 and player2, if one has an entry in the hashmap already, we ignore the points we would add.
                match coop_hm.insert(score.map_data.profile_number1.clone(), 1){
                    Some(_) => (),
                    None => {
                        match final_hm.insert(score.map_data.profile_number1.clone(), Points{
                            points: score.points,
                            score: score.map_data.score,
                            num_scores: 1,
                            total_rank_sum: score.rank,
                            worst: (score.rank, map.clone()),
                            best: (score.rank, map.clone()),
                        }){  
                            Some(pnts) => {final_hm.insert(score.map_data.profile_number1, Points{
                                points: pnts.points + score.points,
                                score: pnts.score + score.map_data.score,
                                num_scores: pnts.num_scores + 1,
                                total_rank_sum: pnts.total_rank_sum + score.rank,
                                worst: if score.rank > pnts.worst.0 {
                                    (score.rank, map.clone())
                                } else{
                                    pnts.worst
                                },
                                best: if score.rank < pnts.best.0{
                                    (score.rank, map.clone())
                                }else {
                                    pnts.best
                                },
                            });},
                            None => (),
                        }
                    },
                } match coop_hm.insert(score.map_data.profile_number2.clone(), 1){
                    Some(_) => (),
                    None => {
                        match final_hm.insert(score.map_data.profile_number2.clone(), Points{
                            points: score.points,
                            score: score.map_data.score,
                            num_scores: 1,
                            total_rank_sum: score.rank,
                            worst: (score.rank, map.clone()),
                            best: (score.rank, map.clone()),
                        }){  
                            Some(pnts) => {final_hm.insert(score.map_data.profile_number2, Points{
                                points: pnts.points + score.points,
                                score: pnts.score + score.map_data.score,
                                num_scores: pnts.num_scores + 1,
                                total_rank_sum: pnts.total_rank_sum + score.rank,
                                worst: if score.rank > pnts.worst.0 {
                                    (score.rank, map.clone())
                                } else{
                                    pnts.worst
                                },
                                best: if score.rank < pnts.best.0{
                                    (score.rank, map.clone())
                                }else {
                                    pnts.best
                                },
                            });},
                            None => (),
                        }
                    }
                }
            }
            // Clear the hashmap so we can reuse the memory allocation for the next map.
            coop_hm.clear();
        }
    }
    // Have this be a map, map all the values into one big hashmap.
    // println!("{:#?}", final_hm.get("76561198039230536"));
    (chapter_id, final_hm)
}


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
