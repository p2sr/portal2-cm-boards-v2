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
    use crate::models::maps::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    let sp = Maps::get_steam_ids(&pool, false).await.unwrap();
    let coop = Maps::get_steam_ids(&pool, true).await.unwrap();
    assert_eq!(sp.len(), 60);
    assert_eq!(coop.len(), 48);
    let map_name = Maps::get_map_name(&pool, sp[0].clone()).await.unwrap().unwrap();
    let pgun = "Portal Gun".to_string();
    assert_eq!(map_name, pgun);
    let default_cat = Maps::get_default_cat(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(default_cat, 49);
    let chapter_id = Maps::get_chapter_from_map_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(chapter_id.id, 7);
    assert_eq!(chapter_id.chapter_name, Some("The Courtesy Call".to_string()));
    let id = Maps::get_steam_id_by_name(&pool, pgun.clone()).await.unwrap();
    assert_eq!(sp[0], id[0]);
    let public = Maps::get_is_public_by_steam_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert!(public);
}

#[actix_web::test]
async fn test_db_chapters() {
    use crate::models::chapters::*;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    let chapter = Chapters {
        id: 7,
        chapter_name: Some("The Courtesy Call".to_string()),
        is_multiplayer: false,
        game_id: 1,
    };
    let map_ids = Chapters::get_map_ids(&pool, chapter.id).await.unwrap();
    assert_eq!(vec!["47458", "47455", "47452", "47106", "47735", "62761", "62758", "62763", "62759"], map_ids);
    let chapters_query_params = ChapterQueryParams {
        chapter_name: Some("the courtesy".to_string()),
        is_multiplayer: Some(false),
        game_id: Some(1),
    };
    let ids = Chapters::get_filtered_chapters(&pool, chapters_query_params).await.unwrap();
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
    use crate::models::users::*;
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
        title: Some("Sysadmin".to_string()),
        admin: 1,
        donation_amount: None,
        discord_id: None,
        auth_hash: None,
        country_id: None,
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
    let test_user = Users::get_user_data(&pool, &user.profile_number).await.unwrap().unwrap();
    assert_eq!(user.board_name, Some(test_user.user_name));
    assert_eq!(user.avatar, Some(test_user.avatar));
    let test_vec = Users::check_board_name(&pool, "Daniel").await.unwrap();
    assert!(test_vec.len() != 0);
    let banned = Users::get_banned(&pool).await.unwrap();
    assert!(banned.len() > 148);
    let banned = Users::check_banned(&pool, &user.profile_number).await.unwrap();
    assert!(!banned);
    let title = Users::get_title(&pool, user.profile_number.clone()).await.unwrap();
    assert_eq!(user.title, title);
    let socials = Users::get_socials(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.twitch, socials.twitch);
    assert_eq!(user.youtube, socials.youtube);
    assert_eq!(user.discord_id, socials.discord_id);
    let admin = Users::get_admin_for_user(&pool, user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(user.admin, admin);
    let admin_vec = Users::get_all_admins(&pool, 1).await.unwrap();
    assert_eq!(admin_vec.len(), 8);
    insert_user.profile_number = "0".to_string();
    
    // Test inserts/updates/deletes
    Users::insert_new_users(&pool, insert_user.clone()).await.unwrap();
    let insert_user_data = Users::get_user_data(&pool, &insert_user.profile_number).await.unwrap().unwrap();
    assert_eq!(insert_user.board_name, Some(insert_user_data.user_name));
    assert_eq!(insert_user.avatar, Some(insert_user_data.avatar));
    insert_user.board_name = Some("BigDaniel11AtlasPog".to_string());
    Users::update_existing_user(&pool, insert_user.clone()).await.unwrap();
    let new_avi = Users::update_avatar(&pool, &user.profile_number, user.avatar.as_ref().unwrap()).await.unwrap();
    assert_eq!(new_avi, user.avatar.unwrap());
    Users::delete_user(&pool, insert_user.profile_number.clone()).await.unwrap();
    let _res = Users::get_user_data(&pool, &insert_user.profile_number).await;

    // Donations
    let donators = Users::get_donators(&pool).await.unwrap();
    assert!(!donators.is_empty());
}

#[actix_web::test]
async fn test_db_demos() {
    use crate::models::demos::*;
    use crate::models::changelog::{Changelog, ChangelogInsert};
    use chrono::NaiveDateTime;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");

    let demo = Demos {
        id: 14607,
        file_id: "LaservsTurret_1763_76561198040982247_14607.dem".to_string(),
        partner_name: None,
        parsed_successfully: true,
        sar_version: None,
        cl_id: 127825,
        updated: None,
    };
    let demo_by_cl_id = Demos::get_demo_by_cl_id(&pool, demo.cl_id).await.unwrap().unwrap();

    let new_demo = Demos::get_demo(&pool, demo_by_cl_id.id).await.unwrap().unwrap();

    assert_eq!(demo_by_cl_id.id, new_demo.id);
    assert_eq!(demo.file_id, new_demo.file_id);
    assert_eq!(demo.partner_name, new_demo.partner_name);
    assert_eq!(demo.parsed_successfully, new_demo.parsed_successfully);
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
    let clinsert = ChangelogInsert {
        timestamp: Some(NaiveDateTime::parse_from_str("2020-10-16 12:11:56", "%Y-%m-%d %H:%M:%S").unwrap()),
        profile_number: "76561198040982247".to_string(),
        score: 1698,
        map_id: "47763".to_string(),
        demo_id: Some(demo_insert),
        banned: false,
        youtube_id: None,
        previous_id: Some(127825),
        coop_id: None,
        post_rank: Some(1),
        pre_rank: Some(3),
        submission: 1,
        note: None,
        category_id: 19,
        score_delta: Some(-65),
        verified: Some(true),
        admin_note: None,
    };
    let mut check_insert = Demos::get_demo(&pool, demo_insert).await.unwrap().unwrap();
    assert_eq!(demo_insert, check_insert.id);
    assert_eq!(new_demo.file_id, check_insert.file_id);
    assert_eq!(new_demo.partner_name, check_insert.partner_name);
    assert_eq!(new_demo.parsed_successfully, check_insert.parsed_successfully);
    assert_eq!(new_demo.sar_version, check_insert.sar_version);
    assert_eq!(new_demo.cl_id, check_insert.cl_id);
    // Testing deleting demos from changelog entries.    
    let new_cl_id = Changelog::insert_changelog(&pool, clinsert.clone()).await.unwrap();
    let new_fid = "Hello World".to_string();
    check_insert.file_id = new_fid.clone();
    // Update the demo
    Demos::update_demo(&pool, check_insert.clone()).await.unwrap();
    let check_updated = Demos::get_demo(&pool, check_insert.id).await.unwrap().unwrap();
    assert_eq!(check_updated.file_id, new_fid);
    // Delete references to the demo entry.
    let updated_cl = Changelog::delete_references_to_demo(&pool, clinsert.demo_id.unwrap()).await.unwrap();
    assert_eq!(updated_cl[0], new_cl_id);
    // Delete the demo entry
    Demos::delete_demo(&pool, check_insert.id).await.unwrap();
    let _res = Demos::get_demo(&pool, check_insert.id).await;
    // Delete the changelog entry
    let _ = Changelog::delete_changelog(&pool, new_cl_id).await.unwrap();
}

#[actix_web::test]
async fn test_db_changelog() {
    use crate::models::changelog::*;
    use chrono::NaiveDateTime;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool");
    let mut transaction = pool.begin().await.unwrap();
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
        submission: 0,
        note: None,
        category_id: 19,
        score_delta: Some(-83),
        verified: Some(true),
        admin_note: None,
        updated: None,
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
        submission: 1,
        note: None,
        category_id: 19,
        score_delta: Some(-65),
        verified: Some(true),
        admin_note: None,
    };
    
    let banned_scores = Changelog::check_banned_scores(&pool, ScoreLookup {
        map_id: "47763".to_string(),
        score: 1763,
        profile_number: "76561198040982247".to_string(),
        cat_id: Some(67),
        game_id: Some(1)
    }).await.unwrap();
    assert!(!banned_scores);
    let pb_history = Changelog::get_sp_pb_history(&pool, "76561198040982247", "47763", 67, 1).await.unwrap();
    assert_ne!(0, pb_history.len());
    let mut new_cl_insert = Changelog::transaction_insert_changelog(&mut transaction, clinsert.clone()).await.unwrap();
    new_cl_insert.note = Some("fat time".to_string());
    let _ = Changelog::transaction_update_changelog(&mut transaction, new_cl_insert.clone()).await.unwrap();
    // let updated_changelog = Changelog::get_changelog(&pool, new_cl_insert.id).await.unwrap().unwrap();
    let _ = Changelog::transaction_delete_changelog(&mut transaction, new_cl_insert.id).await.unwrap();
    let _res = Changelog::get_changelog(&pool, new_cl_insert.id).await;

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
    let cl_page = ChangelogPage::get_changelog_page(&pool, query_params).await.unwrap();
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
    let filtered_cl_page = ChangelogPage::get_changelog_page(&pool, filter).await.unwrap();
    assert_eq!(filtered_cl_page.len(), 1);
    assert_eq!(filtered_cl_page[0].id, 127825);
    transaction.rollback().await.unwrap();
}

#[actix_web::test]
async fn test_db_pages() {
    use crate::models::sp::*;
    use crate::models::coop::*;    
    use crate::tools::helpers::{filter_coop_entries, score};
    let (config, pool) = get_config().await.expect("Error getting config and DB pool");

    let sp_map_id = "47763".to_string();
    let coop_map_id = "52642".to_string();
    let smp = SpMap::get_sp_map_page(&pool, &sp_map_id, DEFAULT_PAGE_SIZE as i32, 67, 1).await.unwrap();
    assert_ne!(smp.len(), 0);
    let cmp = CoopMap::get_coop_map_page(&pool, &coop_map_id, 21, 1).await.unwrap();
    assert_ne!(cmp.len(), 0);
    let coop_entries_filtered = filter_coop_entries(cmp, config.proof.results as usize).await;
    // Ensure we didn't mess up the ranking/points algorithm.
    for i in 0..coop_entries_filtered.len() {
        assert_eq!((i + 1) as i32, coop_entries_filtered[i].rank);
        assert_eq!(score((i + 1) as i32), coop_entries_filtered[i].points);
        if i == 0 {             // Point check for the first entry.
            assert_eq!(200.0, coop_entries_filtered[i].points);
        } else if i == 149 {    // Point check for the 150th entry.
            assert_eq!(13.005, coop_entries_filtered[i].points);
        } else if i == 200 {    // Point check for the 201st entry.
            assert_eq!(0.0, coop_entries_filtered[i].points)
        }
    }

    let sppres = SpPreview::get_sp_previews(&pool).await.unwrap();
    assert_eq!(sppres.len(), 60);
    let cooppres = CoopPreview::get_coop_previews(&pool).await.unwrap();
    assert_eq!(cooppres.len(), 48);

    let _spbanned = SpBanned::get_sp_banned(&pool, sp_map_id).await.unwrap();
    let _coopbanned = CoopBanned::get_coop_banned(&pool, &coop_map_id, 19).await.unwrap();
}

#[actix_web::test]
async fn test_db_admins() {
    use crate::models::admin::*;
    use crate::models::changelog::ChangelogQueryParams;
    let (_, pool) = get_config().await.expect("Error getting config and DB pool.");
    let query_params = ChangelogQueryParams {
        limit: Some(5),
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
    let ban_page = Admin::get_admin_page(&pool, query_params).await.unwrap().unwrap();
    assert!(ban_page.len() == 5);

    let ban_stats = Admin::get_user_banned_time_stats(&pool).await.unwrap().unwrap();
    assert!(!ban_stats.is_empty());
}