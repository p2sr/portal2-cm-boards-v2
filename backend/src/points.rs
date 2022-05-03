use crate::models::{CoopRanked, SpRanked};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsWrapper {
    id: Option<i32>,
    points: HashMap<String, Points>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendWrapper<'a> {
    id: Option<i32>,
    hm_points: &'a HashMap<String, Points>,
    ordered_points: Vec<(String, Points)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points {
    points: f32,
    score: i32, // TODO: Need to change the format to support SAR timing
    num_scores: i32,
    total_rank_sum: i32,
    worst: (i32, String),
    best: (i32, String),
    user_name: Option<String>,
    avatar: Option<String>,
}

impl Points {
    pub fn sum(self, new_points: &Points) -> Points {
        Points {
            points: self.points + new_points.points,
            score: self.score + new_points.score,
            num_scores: self.num_scores + new_points.num_scores,
            total_rank_sum: self.total_rank_sum + new_points.total_rank_sum,
            worst: if new_points.worst.0 > self.worst.0 {
                (new_points.worst.0, new_points.worst.1.clone())
            } else if new_points.worst.0 >= self.worst.0 {
                (self.worst.0, "Multiple".to_string())
            } else {
                self.worst
            },
            best: if new_points.best.0 < self.best.0 {
                (new_points.best.0, new_points.best.1.clone())
            } else if new_points.best.0 <= self.best.0 {
                (self.best.0, "Multiple".to_string())
            } else {
                self.best
            },
            user_name: self.user_name,
            avatar: self.avatar,
        }
    }
}

#[allow(dead_code)]
pub fn score_calc(score: i32) -> (i32, i32, i32) {
    let ms = (score % 100) * 10;
    let seconds = (score / 100) % 60;
    let minutes = score / 100 / 60;
    (ms, seconds, minutes)
}

pub fn calc_points(maps_altered: Option<Vec<i32>>) {
    // NOTE: We could just recalculate points on a set of impacted chapters. We can reuse the cached values for unaffected chapters.
    // If a score update comes in for btg only, we only need to recalc aggtime/aggpoints in chapter 3. But we would still need to update all user profiles? This might save a small amount of time.
    // Additionally, we could also ignore players that do not have scores in that give chapter (very limited # of players, might not be worth the effort).
    let mut overall_hm: HashMap<String, Points> = HashMap::with_capacity(50 * 200);
    let mut sp_hm: HashMap<String, Points> = HashMap::with_capacity(25 * 200);
    let mut coop_hm: HashMap<String, Points> = HashMap::with_capacity(25 * 200);
    if maps_altered == None {
        // Contains a vector of tuples, each hashmap stores the total points for each player, per chapter. Chapters are denoted by the i32 in the tuple.
        // NOTE: The par_iter means we could have the chapters finish calculating in any order, and thus the ordering can not be assumed.
        // par_iter hit endpoint for each chapter (1-6 coop, 7-15 sp)
        let hm_vec: Vec<PointsWrapper> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
            .into_par_iter()
            .map(|chapter_id| {
                let url = format!("http://localhost:8080/api/v1/chapter/{}/maps", &chapter_id);
                let map_ids: Vec<String> = reqwest::blocking::get(&url)
                    .expect("Error in query to our local API (Make sure the webserver is running")
                    .json()
                    .expect("Error in converting our API values to JSON");
                calc_chapter(map_ids, chapter_id)
            })
            .collect();
        // Sort the hashmap by points
        // TODO: Ordered hashmap lib?
        // TODO: This allocation is really annoying, would *love* to fix it. Maybe compute this on the webserver??
        let hm_vec_clone = hm_vec.clone();
        let mut send_vec: Vec<SendWrapper> = Vec::with_capacity(16);
        // TODO: Remove scores of 0.
        for (i, chapter) in hm_vec_clone.into_iter().enumerate() {
            let mut sorted: Vec<_> = chapter.points.into_iter().collect();
            sorted.sort_by(|a, b| b.1.points.partial_cmp(&a.1.points).unwrap());
            send_vec.push(SendWrapper {
                id: chapter.id,
                hm_points: &hm_vec[i].points,
                ordered_points: sorted,
            });
        }

        // Post all chapters to the webserver
        for chapter in send_vec.iter() {
            let client = reqwest::blocking::Client::new();
            let url = "http://localhost:8080/api/v1/points/chapter".to_string();
            client
                .post(&url)
                .json(chapter)
                .send()
                .expect("Error querying our local API");
        }
        // Sp & coop
        for x in hm_vec {
            if x.id.unwrap() > 6 {
                // SP
                for (profile_number, new_points) in x.points {
                    // TODO: Fix this im-> mut-> im pattern using different hashmap methods.
                    match sp_hm.get(&profile_number) {
                        Some(old_points) => {
                            match sp_hm.insert(profile_number.clone(), new_points.sum(old_points)) {
                                //old_points.sum(&new_points)){
                                Some(_) => (),
                                None => unreachable!(),
                            }
                        }
                        None => {
                            match sp_hm.insert(profile_number.clone(), new_points.clone()) {
                                // TODO: Maybe remove clone?
                                Some(_) => unreachable!(),
                                None => (),
                            }
                        }
                    }
                }
            } else {
                // Coop
                for (profile_number, new_points) in x.points {
                    match coop_hm.get(&profile_number) {
                        Some(old_points) => {
                            match coop_hm.insert(profile_number.clone(), new_points.sum(old_points))
                            {
                                Some(_) => (),
                                None => unreachable!(),
                            }
                        }
                        None => match coop_hm.insert(profile_number.clone(), new_points.clone()) {
                            Some(_) => unreachable!(),
                            None => (),
                        },
                    }
                }
            }
        }
        // println!("{:#?}", sp_hm.get("76561198039230536"));
        let sp_hm_clone = sp_hm.clone();
        let mut sorted: Vec<_> = sp_hm_clone.into_iter().collect();
        sorted.sort_by(|a, b| b.1.points.partial_cmp(&a.1.points).unwrap());

        let client = reqwest::blocking::Client::new();
        let url = "http://localhost:8080/api/v1/points/sp".to_string();
        client
            .post(&url)
            .json(&SendWrapper {
                id: None,
                hm_points: &sp_hm,
                ordered_points: sorted,
            })
            .send()
            .expect("Error querying our local API");
        let coop_hm_clone = coop_hm.clone();
        let mut sorted: Vec<_> = coop_hm_clone.into_iter().collect();
        sorted.sort_by(|a, b| b.1.points.partial_cmp(&a.1.points).unwrap());

        let client = reqwest::blocking::Client::new();
        let url = "http://localhost:8080/api/v1/points/coop".to_string();
        client
            .post(&url)
            .json(&SendWrapper {
                id: None,
                hm_points: &coop_hm,
                ordered_points: sorted,
            })
            .send()
            .expect("Error querying our local API");

        // Generate aggregated overall.
        for (profile_number, new_points) in sp_hm {
            match overall_hm.get(&profile_number) {
                Some(old_points) => {
                    match overall_hm.insert(profile_number.clone(), new_points.sum(old_points)) {
                        Some(_) => (),
                        None => unreachable!(),
                    }
                }
                None => match overall_hm.insert(profile_number.clone(), new_points.clone()) {
                    Some(_) => unreachable!(),
                    None => (),
                },
            }
        }
        for (profile_number, new_points) in coop_hm {
            match overall_hm.get(&profile_number) {
                Some(old_points) => {
                    match overall_hm.insert(profile_number.clone(), new_points.sum(old_points)) {
                        Some(_) => (),
                        None => unreachable!(),
                    }
                }
                None => match overall_hm.insert(profile_number.clone(), new_points.clone()) {
                    Some(_) => unreachable!(),
                    None => (),
                },
            }
        }
        let ohmclone = overall_hm.clone();
        let mut sorted: Vec<_> = overall_hm.into_iter().collect();
        sorted.sort_by(|a, b| b.1.points.partial_cmp(&a.1.points).unwrap());

        let client = reqwest::blocking::Client::new();
        let url = "http://localhost:8080/api/v1/points/overall".to_string();
        client
            .post(&url)
            .json(&SendWrapper {
                id: None,
                hm_points: &ohmclone,
                ordered_points: sorted,
            })
            .send()
            .expect("Error querying our local API");
        //println!("{:#?}", overall_hm.get("76561198039230536"));
    } else {
        // Go through all of the maps altered, and refresh points for just those maps.
        // Point calculation certainly doesn't really support being broken up rn.
    }
}

/// Function to go chapter-by-chapter to calculate points.
pub fn calc_chapter(map_ids: Vec<String>, chapter_id: i32) -> PointsWrapper {
    // Keep track of total points/score (time) per chapter
    let mut final_hm: HashMap<String, Points> = HashMap::with_capacity(10 * 200);
    // Make sure to track which players have a score already on a given map (coop)
    let mut coop_hm: HashMap<String, i32> = HashMap::with_capacity(200);
    for map in map_ids.iter() {
        // Grab top X from the web-server for each map.
        if chapter_id > 6 {
            // SP
            let url = format!("http://localhost:8080/api/v1/map/sp/{}", &map).to_string();
            let res: Vec<SpRanked> = reqwest::blocking::get(&url) // Assumes all top 200
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            for score in res {
                // If an entry exists, add this new value to the old value, if not, keep only new value.
                //  score_calc(score.map_data.score);
                match final_hm.insert(
                    score.map_data.profile_number.clone(),
                    Points {
                        points: score.points,
                        score: score.map_data.score,
                        num_scores: 1,
                        total_rank_sum: score.rank,
                        worst: (score.rank, map.clone()),
                        best: (score.rank, map.clone()),
                        user_name: score.map_data.user_name.clone(),
                        avatar: score.map_data.avatar.clone(),
                    },
                ) {
                    Some(pnts) => {
                        final_hm.insert(
                            score.map_data.profile_number,
                            Points {
                                points: pnts.points + score.points,
                                score: pnts.score + score.map_data.score,
                                num_scores: pnts.num_scores + 1,
                                total_rank_sum: pnts.total_rank_sum + score.rank,
                                worst: if score.rank > pnts.worst.0 {
                                    (score.rank, map.clone())
                                } else {
                                    pnts.worst
                                },
                                best: if score.rank < pnts.best.0 {
                                    (score.rank, map.clone())
                                } else {
                                    pnts.best
                                },
                                user_name: score.map_data.user_name,
                                avatar: score.map_data.avatar,
                            },
                        );
                    }
                    None => (),
                }
            }
        } else {
            // Coop
            let url = format!("http://localhost:8080/api/v1/map/coop/{}", &map);
            let res: Vec<CoopRanked> = reqwest::blocking::get(&url) // Assumes all top 200
                .expect("Error in query to our local API (Make sure the webserver is running")
                .json()
                .expect("Error in converting our API values to JSON");
            for score in res {
                //score_calc(score.map_data.score);
                // Do checks for both player 1 and player2, if one has an entry in the hashmap already, we ignore the points we would add.
                match coop_hm.insert(score.map_data.profile_number1.clone(), 1) {
                    Some(_) => (),
                    None => {
                        match final_hm.insert(
                            score.map_data.profile_number1.clone(),
                            Points {
                                points: score.points,
                                score: score.map_data.score,
                                num_scores: 1,
                                total_rank_sum: score.rank,
                                worst: (score.rank, map.clone()),
                                best: (score.rank, map.clone()),
                                user_name: Some(score.map_data.user_name1.clone()),
                                avatar: score.map_data.avatar1.clone(),
                            },
                        ) {
                            Some(pnts) => {
                                final_hm.insert(
                                    score.map_data.profile_number1,
                                    Points {
                                        points: pnts.points + score.points,
                                        score: pnts.score + score.map_data.score,
                                        num_scores: pnts.num_scores + 1,
                                        total_rank_sum: pnts.total_rank_sum + score.rank,
                                        worst: if score.rank > pnts.worst.0 {
                                            (score.rank, map.clone())
                                        } else {
                                            pnts.worst
                                        },
                                        best: if score.rank < pnts.best.0 {
                                            (score.rank, map.clone())
                                        } else {
                                            pnts.best
                                        },
                                        user_name: Some(score.map_data.user_name1.clone()),
                                        avatar: score.map_data.avatar1.clone(),
                                    },
                                );
                            }
                            None => (),
                        }
                    }
                }
                match coop_hm.insert(score.map_data.profile_number2.clone(), 1) {
                    Some(_) => (),
                    None => {
                        match final_hm.insert(
                            score.map_data.profile_number2.clone(),
                            Points {
                                points: score.points,
                                score: score.map_data.score,
                                num_scores: 1,
                                total_rank_sum: score.rank,
                                worst: (score.rank, map.clone()),
                                best: (score.rank, map.clone()),
                                user_name: score.map_data.user_name2.clone(),
                                avatar: score.map_data.avatar2.clone(),
                            },
                        ) {
                            Some(pnts) => {
                                final_hm.insert(
                                    score.map_data.profile_number2,
                                    Points {
                                        points: pnts.points + score.points,
                                        score: pnts.score + score.map_data.score,
                                        num_scores: pnts.num_scores + 1,
                                        total_rank_sum: pnts.total_rank_sum + score.rank,
                                        worst: if score.rank > pnts.worst.0 {
                                            (score.rank, map.clone())
                                        } else {
                                            pnts.worst
                                        },
                                        best: if score.rank < pnts.best.0 {
                                            (score.rank, map.clone())
                                        } else {
                                            pnts.best
                                        },
                                        user_name: score.map_data.user_name2,
                                        avatar: score.map_data.avatar2,
                                    },
                                );
                            }
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
    // The placeholder "N/A" profile_number is getting their points calculated. This removes the entry.
    final_hm.remove("N/A");
    PointsWrapper {
        id: Some(chapter_id),
        points: final_hm,
    }
}

// Algorithm (TO BE IMPROVED)
//  Scores: Pull all top 200 score data for current maps, break into different threads by chapter.
//  Chapter:
//  Create a hashmap, with key profile_number, value is a struct that contains data for all chapters (each chapter has a score and time).
//  NOTE: For concurrency, we might need to unsafe wrap, or do other shit to ensure that we can mutate the same struct instance accross multiple threads.
//  In theory, this should be okay, because each thread will only have mutable access to specific compontents of the struct.
//      Overall:
//      SP
//          Sum all sp chapters.
//      Coop
//          Sum all coop chapters.
//      Overall
//          Sum both sp/coop.
//          Cache
//      Player Profile / Stats:
//          Stats
//          # wrs
//      Points
//      Position
//      Avg placement
//      Best/worst
//      Newest/oldest
//      Scores
// All score history (break this into smaller calls?), all aggregated time/points history.
