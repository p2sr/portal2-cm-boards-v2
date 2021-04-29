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

mod stages;
use stages::exporting::*;
use stages::fetching::*;

mod models;

fn main() {
    dotenv().ok();
    //let database_url = env::var("DATABASE_URL").expect("set the DATABASE_URL in ../.env");
    //let conn = MysqlConnection::establish(&database_url).unwrap();
    fetch_sp();


}

fn fetch_sp(){
    let official_sp = [47458,47455,47452,47106,47735,47736,47738,47742,
    47744,47465,47746,47748,47751,47752,47755,47756,47759,47760,47763,47764,47766,47768,47770,
    47773,47774,47776,47779,47780,47783,47784,47787,47468,47469,47472,47791,47793,47795,47798,
    47800,47802,47804,47806,47808,47811,47813,47815,47817,47819,47821,47824,47456];

    
    // for entry in official_sp.iter(){
    //     let res = fetch_entries(*entry, 0, 200);
    //     // println!("{:#?}",res);
    // }
    let utc: DateTime<Utc> = Utc::now();
   
    let res: Vec<_> = official_sp.into_par_iter().map(|entry|{
        fetch_entries(*entry, 0, 450, utc)
    }).collect();
    // What do we do with the leaderboards...


}
/*
changelog (id) {
    profile_number -> Varchar,          -> taken care of
    score -> Integer,                   -> taken care of
    map_id -> Varchar,                  -> taken care of
    wr_gain -> Integer,                 -> need to calculate
    previous_id -> Nullable<Integer>,   -> need to calculate
    post_rank -> Nullable<Integer>,     -> need to calculate
    pre_rank -> Nullable<Integer>,      -> need to calculate
    has_demo -> Nullable<Integer>,      -> always 0
    banned -> Integer,                  -> always 0
    submission -> Integer,              -> always 0
    youtube_id -> Nullable<Varchar>,    -> always NULL
    coopid -> Nullable<Integer>,        -> always NULL for SP
    note -> Nullable<Varchar>,          -> always NULL
    category -> Nullable<Varchar>,      -> always "any%"
    id -> Integer,                      -> auto inc
}
*/