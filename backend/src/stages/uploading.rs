use crate::models::datamodels::AvatarInsert;
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
