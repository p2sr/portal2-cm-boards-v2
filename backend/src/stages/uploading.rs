use crate::models::{AvatarInsert, Changelog};
use crate::models::{ChangelogInsert, DemoInsert, DemoOptions, GetPlayerSummariesWrapper, Users};
use crate::update_image;
use anyhow::Result;

pub fn upload_new_pfp(profile_number: &str) -> Result<String> {
    let avatar = update_image(profile_number)?;
    let post_url = format!(
        "http://localhost:8080/api/v1/user/avatar/{}",
        profile_number
    );
    Ok(reqwest::blocking::Client::new()
        .put(&post_url)
        .json(&AvatarInsert { avatar: &avatar })
        .send()?
        .json::<String>()?)
}

// TODO: 620 - Portal 2.
// http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}}&format=json
pub async fn add_user(profile_number: &str) -> Result<Users> {
    // http://steamcommunity.com/profiles/{}/?xml=1
    // GET https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/
    let api_key = dotenv::var("STEAM_API_KEY").expect("Cannot find STEAM_API_KEY in ./.env");

    let steam_api_url = format!(
        "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&steamids={}",
        api_key, profile_number
    );
    let user = reqwest::get(&steam_api_url)
        .await?
        .json::<GetPlayerSummariesWrapper>()
        .await?;

    let new_user = Users {
        profile_number: profile_number.to_string(),
        board_name: None,
        steam_name: Some(user.response.players[0].personaname.clone()),
        banned: false,
        registered: 0,
        avatar: Some(user.response.players[0].avatarfull.clone()),
        ..Default::default()
    };

    let url = String::from("http://localhost:8080/api/v1/user");
    let client = reqwest::Client::new();
    Ok(client
        .post(&url)
        .json(&new_user)
        .send()
        .await?
        .json::<Users>()
        .await?)
}

pub async fn upload_changelog_and_demo(cl: &ChangelogInsert, demo: &DemoInsert) -> Result<i64> {
    let client = reqwest::Client::new();
    let new_id = client
        .post("http://localhost:8080/api/v1/sp/post_score")
        .json(cl)
        .send()
        .await?
        .json::<i64>()
        .await?;
    println!("Successfully uploaded time {new_id}");
    let demo_id = client // Upload the demo
        .post("http://localhost:8080/api/v1/demos")
        .json(demo)
        .send()
        .await?
        .json::<i64>()
        .await?;
    println!("Successfully upload demo {demo_id}");
    client
        .put("http://localhost:8080/api/v1/changelog/demo")
        .json(&DemoOptions {
            demo_id,
            cl_id: new_id,
        })
        .send()
        .await?
        .json::<Changelog>()
        .await?;
    Ok(new_id)
}
