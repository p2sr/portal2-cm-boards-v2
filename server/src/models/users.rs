use sqlx::FromRow;

/// One-to-one struct for user data.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Users {
    pub profile_number: String,
    pub board_name: Option<String>,
    pub steam_name: Option<String>,
    pub banned: bool,
    pub registered: i32,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
    pub discord_id: Option<String>,
    pub auth_hash: Option<String>,
    pub country_id: Option<i32>,
}

/// One-to-one struct for countries
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Countries {
    id: i32,
    iso: String,
    name: String,
    nicename: String,
    iso3: String,
    numcode: i32,
    phonecode: i32,
}

/// One-to-one struct for countries
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct CountriesMin {
    id: i32,
    iso3: String,
    nicename: String,
}

/// Includes only a `user_name` and `avatar`, does not include the `profile_number`
///
/// Used for when the `profile_number` is included in another portion of the returned values.
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct UsersPage {
    pub user_name: String,
    pub avatar: String,
}

/// Wraps `profile_number`, `user_name` and `avatar` for displaying a user.
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct UsersDisplay {
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct UsersDisplayCount {
    pub profile_number: String,
    pub user_name: String,
    pub avatar: String,
    pub count: i64,
}

/// Social media accounts from `Users`
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Socials {
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub discord_id: Option<String>,
}

/// An avatar being added to the db.
#[derive(Clone, Debug, Deserialize)]
pub struct AvatarInsert {
    pub avatar: String,
}

/// Wrapper for our API call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPlayerSummariesWrapper {
    pub response: Players,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Players {
    pub players: Vec<GetPlayerSummaries>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPlayerSummaries {
    pub steamid: String,
    pub communityvisibilitystate: i32,
    pub profilestate: i32,
    pub personaname: String,
    pub lastlogoff: i32,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
}
