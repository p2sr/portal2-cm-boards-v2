#[cfg(test)]
#[test]
/// Tests to make sure data can be pulled from the steam API.
fn test_steam_api() {
    use crate::stages::fetching::*;

    let image: String = update_image("76561198040982247");
    // println!("{}", image);
    assert_eq!(image, "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/92/921d9d7402a6e766759bcc0b2ac7b91f1dcf0ad2_full.jpg".to_string());
}

#[test]
#[should_panic] // Should panic, we're uploading a duplicate user.
/// Check the uploading endpoint.
/// REQUIRES THAT THE LOCAL WEB SERVER AND DB ARE UP.
fn add_user_steam_api() {
    use crate::stages::fetching::*;

    match add_user("76561198040982247".to_string()) {
        Ok(_) => (),
        Err(e) => {
            // Could not insert user, it already exists.
            panic!("{}", e);
        }
    }
}

#[test]
/// Ensure we get the correct response when checking to make sure a user exists.
/// REQUIRES THAT THE LOCAL WBE SERVER AND DB ARE UP. `
fn check_user() {
    use crate::stages::fetching::*;
    assert!(check_user("76561198040982247"));
}
