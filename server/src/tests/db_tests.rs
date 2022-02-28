// use actix_web::{test, HttpRequest, HttpResponse, HttpMessage};
// use actix_web::http::{header, StatusCode};
use sqlx::PgPool;
use dotenv::dotenv;
use anyhow::Result;
use crate::tools::config::Config;

#[allow(dead_code)]
const DEFAULT_PAGE_SIZE: usize = 500;

#[allow(dead_code)]
async fn get_config() -> Result<(Config, PgPool)> {
    dotenv().ok();
    let config = Config::from_env()?;
    let pool = PgPool::connect(&config.database_url).await?;
    Ok((config, pool))
}

#[actix_web::test]
async fn test_db_maps() {
    use crate::controllers::models::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    let sp = Maps::get_steam_ids(&pool, false).await.unwrap();
    let coop = Maps::get_steam_ids(&pool, true).await.unwrap();
    assert_eq!(sp.len(), 60);
    assert_eq!(coop.len(), 48);
    let map_name = Maps::get_map_name(&pool, sp[0].clone()).await.unwrap().unwrap();
    let pgun = "Portal Gun".to_string();
    assert_eq!(map_name, pgun);
    let default_cat = Maps::get_default_cat(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(default_cat, 1);
    let chapter_id = Maps::get_chapter_from_map_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(chapter_id.id, 7);
    assert_eq!(chapter_id.chapter_name, Some("The Courtesy Call".to_string()));
    let id = Maps::get_steam_id_by_name(&pool, pgun.clone()).await.unwrap().unwrap();
    assert_eq!(sp[0], id[0]);
    let public = Maps::get_is_public_by_steam_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert!(public);
}

#[actix_web::test]
async fn test_db_chapters() {
    use crate::controllers::models::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    let chapter = Chapters {
        id: 7,
        chapter_name: Some("The Courtesy Call".to_string()),
        is_multiplayer: false,
        game_id: 1,
    };
    let map_ids = Chapters::get_map_ids(&pool, chapter.id).await.unwrap().unwrap();
    assert_eq!(vec!["47458", "47455", "47452", "47106", "47735", "62761", "62758", "62763", "62759"], map_ids);
    let ids = Chapters::get_chapter_by_name(&pool, "The Courtesy Call".to_string()).await.unwrap().unwrap();
    assert_eq!(7, ids[0].id);
    let new_chapter = Chapters::get_chapter_by_id(&pool, chapter.id).await.unwrap().unwrap();
    assert_eq!(chapter.id, new_chapter.id);
    assert_eq!(chapter.chapter_name, new_chapter.chapter_name);
    assert_eq!(chapter.is_multiplayer, new_chapter.is_multiplayer);
    assert_eq!(chapter.game_id, new_chapter.game_id);
    let is_mp = Chapters::get_chapter_is_multiplayer(&pool, chapter.id).await.unwrap().unwrap();
    assert_eq!(chapter.is_multiplayer, is_mp);
    let game = Chapters::get_chapter_game(&pool, chapter.id).await.unwrap().unwrap();
    assert_eq!(chapter.game_id, game.id);
}


// TODO: We want to make this prone to handling changes, right now many of the tests
//       are hard-coded to only work on this current version of the db.
#[actix_web::test]
async fn test_db_users() {
    use crate::controllers::models::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    
    let user: Users = Users{ 
        profile_number: "76561198040982247".to_string(),
        board_name: Some("Daniel".to_string()),
        steam_name: Some("BigDaniel".to_string()),
        banned: false,
        registered: 0,
        avatar: Some("https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg".to_string()),
        twitch: Some("bigdaniel".to_string()),
        youtube: Some("/channel/UCtwF46_PUGCefgRfrcIXOZA".to_string()),
        title: None,
        admin: 1,
        donation_amount: None,
        discord_id: None,
    };
    let mut insert_user = user.clone();
    let test_user = Users::get_user(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.profile_number, test_user.profile_number);
    assert_eq!(user.board_name, test_user.board_name);
    assert_eq!(user.steam_name, test_user.steam_name);
    assert_eq!(user.banned, test_user.banned);
    assert_eq!(user.registered, test_user.registered);
    assert_eq!(user.avatar, test_user.avatar);
    assert_eq!(user.twitch, test_user.twitch);
    assert_eq!(user.youtube, test_user.youtube);
    assert_eq!(user.title, test_user.title);
    assert_eq!(user.admin, test_user.admin);
    assert_eq!(user.donation_amount, test_user.donation_amount);
    assert_eq!(user.discord_id, test_user.discord_id);
    let test_user = Users::get_user_data(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.board_name, Some(test_user.user_name));
    assert_eq!(user.avatar, Some(test_user.avatar));
    let test_vec = Users::check_board_name(&pool, "Daniel".to_string()).await.unwrap().unwrap();
    assert_eq!(test_vec.len(), 3);
    assert_eq!(test_vec[0], "76561197960354819");
    assert_eq!(test_vec[1], "76561198040982247");
    assert_eq!(test_vec[2], "76561198057122387");
    let banned = Users::get_banned(&pool).await.unwrap();
    assert_eq!(banned.len(), 148);
    let banned = Users::check_banned(&pool, user.profile_number.clone()).await.unwrap();
    assert!(!banned);
    let title = Users::get_title(&pool, user.profile_number.clone()).await.unwrap();
    assert_eq!(user.title, title);
    let socials = Users::get_socials(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.twitch, socials.twitch);
    assert_eq!(user.youtube, socials.youtube);
    assert_eq!(user.discord_id, socials.discord_id);
    let admin = Users::get_admin_for_user(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.admin, admin);
    let admin_vec = Users::get_all_admins(&pool, 1).await.unwrap().unwrap();
    assert_eq!(admin_vec.len(), 8);
    assert_eq!(admin_vec[7].user_name, "Lathil".to_string());
    insert_user.profile_number = "0".to_string();
    // Test inserts/updates/deletes
    assert!(Users::insert_new_users(&pool, insert_user.clone()).await.unwrap());
    let insert_user_data = Users::get_user_data(&pool, insert_user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(insert_user.board_name, Some(insert_user_data.user_name));
    assert_eq!(insert_user.avatar, Some(insert_user_data.avatar));
    insert_user.board_name = Some("BigDaniel11AtlasPog".to_string());
    assert!(Users::update_existing_user(&pool, insert_user.clone()).await.unwrap());
    assert!(Users::delete_user(&pool, insert_user.profile_number.clone()).await.unwrap());
    let _res = Users::get_user_data(&pool, insert_user.profile_number.clone()).await;
}

#[actix_web::test]
async fn test_db_demos() {
    use crate::controllers::models::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");

    let demo = Demos {
        id: 14598,
        file_id: "LaservsTurret_1763_76561198040982247_14598.dem".to_string(),
        partner_name: None,
        parsed_successfully: true,
        sar_version: None,
        cl_id: 127825
    };
    let new_demo = Demos::get_demo(&pool, demo.id).await.unwrap().unwrap();
    assert_eq!(demo.id, new_demo.id);
    assert_eq!(demo.file_id, new_demo.file_id);
    assert_eq!(demo.partner_name, new_demo.partner_name);
    assert_eq!(demo.parsed_successfully, new_demo.parsed_successfully);
    // TODO: All sar_version values are empty strings for some reason...
    assert_eq!(demo.sar_version, new_demo.sar_version);
    assert_eq!(demo.cl_id, new_demo.cl_id);

    let fid = Demos::get_demo_file_id(&pool, demo.id).await.unwrap().unwrap();
    assert_eq!(demo.file_id, fid);
    let partner_name = Demos::get_partner_name(&pool, demo.id).await.unwrap();
    assert_eq!(demo.partner_name, partner_name);
    let parsed = Demos::check_parsed(&pool, demo.id).await.unwrap();
    assert_eq!(demo.parsed_successfully, parsed);
    let sar_version = Demos::get_sar_version(&pool, demo.id).await.unwrap();
    assert_eq!(demo.sar_version, sar_version);
    let new_demo = DemoInsert {
        file_id: "Doors_831_76561198039230536.dem".to_string(),
        partner_name: Some("Undead".to_string()),
        parsed_successfully: false,
        sar_version: Some("12.7.2-pre".to_string()),
        cl_id: 1,
    };
    let demo_insert = Demos::insert_demo(&pool, new_demo.clone()).await.unwrap();
    let mut check_insert = Demos::get_demo(&pool, demo_insert).await.unwrap().unwrap();
    assert_eq!(demo_insert, check_insert.id);
    assert_eq!(new_demo.file_id, check_insert.file_id);
    assert_eq!(new_demo.partner_name, check_insert.partner_name);
    assert_eq!(new_demo.parsed_successfully, check_insert.parsed_successfully);
    assert_eq!(new_demo.sar_version, check_insert.sar_version);
    assert_eq!(new_demo.cl_id, check_insert.cl_id);
    let new_fid = "Hello World".to_string();
    check_insert.file_id = new_fid.clone();
    assert!(Demos::update_demo(&pool, check_insert.clone()).await.unwrap());
    let check_updated = Demos::get_demo(&pool, check_insert.id).await.unwrap().unwrap();
    assert_eq!(check_updated.file_id, new_fid);
    assert!(Demos::delete_demo(&pool, check_insert.id).await.unwrap());
    let _res = Demos::get_demo(&pool, check_insert.id).await;
}

#[actix_web::test]
async fn test_db_changelog() {
    use crate::controllers::models::*;
    use chrono::NaiveDateTime;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    #[allow(unused_variables)]
    let changelog = Changelog {
        id: 127825,
        timestamp: Some(NaiveDateTime::parse_from_str("2020-10-16 12:11:56", "%Y-%m-%d %H:%M:%S").unwrap()),
        profile_number: "76561198040982247".to_string(),
        score: 1763,
        map_id: "47763".to_string(),
        demo_id: Some(14598),
        banned: false,
        youtube_id: Some("-InZK6yZb08?start=0".to_string()),
        previous_id: Some(113132),
        coop_id: None,
        post_rank: Some(1),
        pre_rank: Some(3),
        submission: false,
        note: None,
        category_id: 19,
        score_delta: Some(-83),
        verified: Some(true),
        admin_note: None,
    };

    let clinsert = ChangelogInsert {
        timestamp: Some(NaiveDateTime::parse_from_str("2020-10-16 12:11:56", "%Y-%m-%d %H:%M:%S").unwrap()),
        profile_number: "76561198040982247".to_string(),
        score: 1698,
        map_id: "47763".to_string(),
        demo_id: None,
        banned: false,
        youtube_id: None,
        previous_id: Some(127825),
        coop_id: None,
        post_rank: Some(1),
        pre_rank: Some(3),
        submission: true,
        note: None,
        category_id: 19,
        score_delta: Some(-65),
        verified: Some(true),
        admin_note: None,
    };

    let banned_scores = Changelog::check_banned_scores(&pool, "47763".to_string(), 1763, "76561198040982247".to_string(), 19).await.unwrap();
    assert!(!banned_scores);
    let pb_history = Changelog::get_sp_pb_history(&pool, "76561198040982247".to_string(), "47763".to_string()).await.unwrap();
    assert_eq!(11, pb_history.len());
    let new_cl_id = Changelog::insert_changelog(&pool, clinsert.clone()).await.unwrap();
    let mut new_cl = Changelog::get_changelog(&pool, new_cl_id).await.unwrap().unwrap();
    new_cl.note = Some("fat time".to_string());
    let is_updated = Changelog::update_changelog(&pool, new_cl.clone()).await.unwrap();
    assert!(is_updated);
    let updated_changelog = Changelog::get_changelog(&pool, new_cl_id).await.unwrap().unwrap();
    assert_eq!(new_cl.id, updated_changelog.id);
    assert_eq!(new_cl.timestamp, updated_changelog.timestamp);
    assert_eq!(new_cl.score, updated_changelog.score);
    assert_eq!(new_cl.map_id, updated_changelog.map_id);
    assert_eq!(new_cl.demo_id, updated_changelog.demo_id);
    assert_eq!(new_cl.banned, updated_changelog.banned);
    assert_eq!(new_cl.youtube_id, updated_changelog.youtube_id);
    assert_eq!(new_cl.previous_id, updated_changelog.previous_id);
    assert_eq!(new_cl.coop_id, updated_changelog.coop_id);
    assert_eq!(new_cl.post_rank, updated_changelog.post_rank);
    assert_eq!(new_cl.pre_rank, updated_changelog.pre_rank);
    assert_eq!(new_cl.submission, updated_changelog.submission);
    assert_eq!(Some("fat time".to_string()), updated_changelog.note);
    assert_eq!(new_cl.category_id, updated_changelog.category_id);
    assert_eq!(new_cl.score_delta, updated_changelog.score_delta);
    assert_eq!(new_cl.verified, updated_changelog.verified);
    assert_eq!(new_cl.admin_note, updated_changelog.admin_note);
    let deleted = Changelog::delete_changelog(&pool, new_cl_id).await.unwrap();
    assert!(deleted);
    let _res = Changelog::get_changelog(&pool, new_cl_id).await;

    let query_params = ChangelogQueryParams {
        limit: Some(500),
        nick_name: None,
        profile_number: None,
        chamber: None,
        sp: None,
        coop: None,
        wr_gain: None,
        has_demo: None,
        yt: None,
        first: None,
        last: None,
    };

    // ChangelogPage
    let cl_page = ChangelogPage::get_changelog_page(&pool, query_params).await.unwrap().unwrap();
    assert_eq!(cl_page.len(), DEFAULT_PAGE_SIZE);
    let filter = ChangelogQueryParams {
        limit: Some(200),
        nick_name: Some("Daniel".to_string()),
        profile_number: None,
        chamber: Some("47763".to_string()),
        sp: Some(true),
        coop: Some(true),
        wr_gain: Some(true),
        has_demo: Some(true),
        yt: None,
        first: None,
        last: None,
    };
    let filtered_cl_page = ChangelogPage::get_changelog_page(&pool, filter).await.unwrap().unwrap();
    assert_eq!(filtered_cl_page.len(), 1);
    assert_eq!(filtered_cl_page[0].id, 127825);
}

#[actix_web::test]
async fn test_db_pages() {
    use crate::controllers::models::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");

    let sp_map_id = "47763".to_string();
    let coop_map_id = "52642".to_string();
    let smp = SpMap::get_sp_map_page(&pool, sp_map_id.clone(), DEFAULT_PAGE_SIZE as i32, 19).await.unwrap();
    assert_ne!(smp.len(), 0);
    let cmp = CoopMap::get_coop_map_page(&pool, coop_map_id.clone(), DEFAULT_PAGE_SIZE as i32, 81).await.unwrap();
    assert_ne!(cmp.len(), 0);

    let sppres = SpPreviews::get_sp_previews(&pool).await.unwrap();
    assert_eq!(sppres.len(), 60);
    let cooppres = CoopPreviews::get_coop_previews(&pool).await.unwrap();
    assert_eq!(cooppres.len(), 48);

    let _spbanned = SpBanned::get_sp_banned(&pool, sp_map_id).await.unwrap();
    let _coopbanned = CoopBanned::get_coop_banned(&pool, coop_map_id, 19).await.unwrap();
}