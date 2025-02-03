# Building

The backend now works as a REST API that allows for updating of scores from the official steam leaderboards, as well as point calculations and avatar updating. It handles direct interactions with Valve's API.

The backend does not rely on a database, but does depend on the webserver running to be able to pull information about the current state of the boards.

You're required to have a steam API key, apply for one [here](https://steamcommunity.com/dev/apikey).
Copy the `.env.example` file in the `/backend` folder, and rename it to `.env`, then fill out the steam_api_key value.

## .env Example

```txt
STEAM_API_KEY=8U0SG8SDG7S0DHISD0FHS0DV7SD
```

Assuming the web server is running locally, run with `cargo run` in `/backend`

## Features

* Update the web server with scores from the steam leaderboard.
* Update profile images for users from the steam API.
* Create new users when their score is added to the leaderboards.
* Calculate the points to display on the leaderboards.

## Planned

* Build in dev vs prod path for endpoints to hit.
