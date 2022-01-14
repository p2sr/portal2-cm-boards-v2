#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(mutable_borrow_reservation_conflict)]

#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use time::PreciseTime;

use chrono::prelude::*;
use dotenv::dotenv;
use log::{info, trace, warn};
use rayon::prelude::*;

mod stages;
use stages::exporting::*;
use stages::fetching::*;

mod points;
use points::*;

mod models;

fn main() {
    // Look into clap? https://docs.rs/clap/2.33.3/clap/
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
    let start = PreciseTime::now();
    dotenv().ok();
    if args.len() == 1 {
        unimplemented!();
        //fetch_all()
    } else if args.len() == 2 {
        match args.get(1) {
            Some(a) => match a.as_str() {
                "rcp" => calc_points(None),
                _ => panic!("Incorrect value"),
            },
            None => panic!("Incorrect Value"),
        }
    } else if args.len() == 3 {
        match args.get(1) {
            Some(a) => match a.as_str() {
                "ssp" => fetch_sp(args.get(2).expect("Invalid map_id for arg #2").to_string()),
                "scp" => fetch_cp(args.get(2).expect("Invalid map_id for arg #2").to_string()),
                "rcp" => calc_points(None),
                _ => panic!("Incorrect value"),
            },
            None => panic!("Incorrect value"),
        }
    } else {
        panic!("Incorrect arg #");
    }
    let end = PreciseTime::now();
    println!("{}", start.to(end));
}

fn fetch_all() {
    let official_sp = [
        47458, 47455, 47452, 47106, 47735, 47736, 47738, 47742, 47744, 47465, 47746, 47748, 47751,
        47752, 47755, 47756, 47759, 47760, 47763, 47764, 47766, 47768, 47770, 47773, 47774, 47776,
        47779, 47780, 47783, 47784, 47787, 47468, 47469, 47472, 47791, 47793, 47795, 47798, 47800,
        47802, 47804, 47806, 47808, 47811, 47813, 47815, 47817, 47819, 47821, 47824, 47456,
    ];

    let official_coop = [
        47741, 47825, 47828, 47829, 45467, 46362, 47831, 47833, 47835, 47837, 47840, 47841, 47844,
        47845, 47848, 47849, 47854, 47856, 47858, 47861, 52642, 52660, 52662, 52663, 52665, 52667,
        52671, 52687, 52689, 52691, 52777, 52694, 52711, 52714, 52715, 52717, 52735, 52738, 52740,
        49341, 49343, 49345, 49347, 49349, 49351, 52757, 52759, 48287,
    ];

    let utc = Utc::now().naive_utc();
    let res_sp: Vec<_> = official_sp
        .into_par_iter()
        .map(|map_id| {
            // TODO: Pass values like # of results as args to the binary
            fetch_entries(map_id, 0, 450, utc, false)
        })
        .collect();
    let res_cp: Vec<_> = official_coop
        .into_par_iter()
        .map(|map_id| fetch_entries(map_id, 0, 800, utc, true))
        .collect();

    // What do we do with the leaderboards...
}

fn fetch_sp(map_id: String) {
    let utc = Utc::now().naive_utc();
    let res_sp = fetch_entries(
        map_id.parse().expect("Error parsing map_id"),
        0,
        450,
        utc,
        false,
    );
    // Recalculate the points on the given map. Force reset cache on webserver.
    // Setup an endpoint on the webserver to invalidate cache for a specific map.
}
fn fetch_cp(map_id: String) {
    let utc = Utc::now().naive_utc();
    let res_coop = fetch_entries(
        map_id.parse().expect("Error parsing map_id"),
        0,
        800,
        utc,
        true,
    );
}
