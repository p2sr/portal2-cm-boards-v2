// use actix_web::{test, HttpRequest, HttpResponse, HttpMessage};
// use actix_web::http::{header, StatusCode};
use sqlx::PgPool;
use dotenv::dotenv;
use anyhow::Result;
use crate::tools::config::Config;

#[allow(dead_code)]
async fn get_config() -> Result<(Config, PgPool)> {
    dotenv().ok();
    let config = Config::from_env()?;
    let pool = PgPool::connect(&config.database_url).await?;
    Ok((config, pool))
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
    assert_eq!(banned, false);
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
    assert_eq!(Users::insert_new_users(&pool, insert_user.clone()).await.unwrap(), true);
    let insert_user_data = Users::get_user_data(&pool, insert_user.profile_number.clone()).await.unwrap().unwrap();
    assert_eq!(insert_user.board_name, Some(insert_user_data.user_name));
    assert_eq!(insert_user.avatar, Some(insert_user_data.avatar));
    insert_user.board_name = Some("BigDaniel11AtlasPog".to_string());
    assert_eq!(Users::update_existing_user(&pool, insert_user.clone()).await.unwrap(), true);
    assert_eq!(Users::delete_user(&pool, insert_user.profile_number.clone()).await.unwrap(), true);
    let _res = Users::get_user_data(&pool, insert_user.profile_number.clone()).await;
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
    let default_cat = Maps::get_deafult_cat(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(default_cat, 1);
    let chapter_id = Maps::get_chapter_from_map_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(chapter_id.id, 7);
    assert_eq!(chapter_id.chapter_name, Some("The Courtesy Call".to_string()));
    let id = Maps::get_steam_id_by_name(&pool, pgun.clone()).await.unwrap().unwrap();
    assert_eq!(sp[0], id[0]);
    let public = Maps::get_is_public_by_steam_id(&pool, sp[0].clone()).await.unwrap().unwrap();
    assert_eq!(true, public);
}