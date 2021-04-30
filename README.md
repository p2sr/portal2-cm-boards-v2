# Portal 2 Leaderboards Revision (board.portal2.sr)

## Building

### Backend
The backend binary can be build by using `cargo build` in the `backend` directory. With Rust installed, it should download all dependancies and compile the binary for you.
#### Features
* Pulling Official Single Player Map data from Steam, caching that data to avoid needing to re-parse/compare.
* Supports multithreading with Rayon.
* Queries to the API for comparison data.
#### Future Plans
Plans for the future include allowing args for the binary to allow it to distinguish between different tasks, but the current goal is to use it for fetching / process / cache times from the 
Steam Leaderboards, as well as to calculate and cache profile data for all players.

The purpose of keeping this backend seperate from the web-server is to off-load some more computationally heavy tasks to an entirely different process for modularity. None of this design is final
### Server
The server can be started with `cargo run` in the `server` directory. You will need diesel installed, and a connection to your MySQL server. You should run migration setup for diesel
so the Portal 2 Leaderboards are populated in MySQL for you. 

Be sure to copy the `.env.example` file, remove `.example` from the file name, and change the contents of the file to suite your usecase.
#### Features:
* Endpoints for sp/coop/changelog pages.
* Supports db pool and async for non-blocking, fast response to simultanious queries.
#### Future Plans
* Result Caching.
* Database / Filtering reworks pending changes to [diesel.rs](https://github.com/diesel-rs/diesel).
* Authentication handling through Steam.
* Permissions handling for Admin users.
* Category integration.
* Player Profiles.

### Front-end
The front-end build can be done with `npm install` in the `/board-portal-2` folder and once dependancies are installed, the client server can be started with `npm start`
#### Features
* Supports querying a running webserver on it's given endpoints for changelog, preview pages, sp maps and coop maps.
* Light and dark theme support.
* Prototyped page designs for many auxiliary pages.

## Original Project Team
This project started as a Senior Capstone Project for the following members.
* @DanielBatesJ
* @MitchellBaker65
* @Pixel-Knight
* @MurphyMichael
* @JFiedler23

## Project References 
[Original Portal 2 Boards](https://github.com/iVerb1/Portal2Boards)

[Least Portals Github](https://github.com/NeKzor/lp)

