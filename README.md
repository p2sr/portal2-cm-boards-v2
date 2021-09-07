# Portal 2 Leaderboards Revision (board.portal2.sr)


## Docker Setup
1. Pull this repo into a local directory. (Set up .env, see .env Example below)
2. Have docker installed.
3. Run `docker-compose up -d`, this should setup a db migration and start a psql db container.
4. Run `docker-compose run --rm web-server`, should connect you to the Linux container that hosts the Rust development environment.
    1. Inside of this container, you can `cd` into the `server` directory, and build the webserver with `cargo run`
    2. The `/server` folder is mounted, meaning you can edit your local repo (using your favorite editor), and the changes will propagate. 

## Backend
### Building (Without Docker)
The backend binary can be build by using `cargo build` in the `backend` directory. With Rust installed, it should download all dependancies and compile the binary for you.
#### Features
* Pulling Official Single Player Map data from Steam, caching that data to avoid needing to re-parse/compare.
* Supports multithreading with Rayon.
* Queries to the API for comparison data.
#### Future Plans
Plans for the future include allowing args for the binary to allow it to distinguish between different tasks, but the current goal is to use it for fetching / process / cache times from the 
Steam Leaderboards, as well as to calculate and cache profile data for all players.

The purpose of keeping this backend seperate from the web-server is to off-load some more computationally heavy tasks to an entirely different process for modularity. None of this design is final

## Server
### Building (Without Docker)
The code is being re-writen to no longer use Diesel.rs and MySQL. More information to come.

**Be sure to copy the `.env.example` file, remove `.example` from the file name, and change the contents of the file to suite your usecase.**

#### .env Example

```
DATABASE.DATABASE_URL=postgresql://username:password@postgres:5432/p2boards
SERVER.HOST=127.0.0.1
SERVER.PORT=8080
PROOF.DEMO=80
PROOF.VIDEO=100
RUST_LOG=1
RUST_LOG="actix_web=info"
```

#### Features:
* Endpoints for sp/coop/changelog pages.
* Supports db pool and async for non-blocking, fast response to simultanious queries.
#### Future Plans
* Result Caching (redis?).
* Migrating to Postgres.
* Different query handler.
* Authentication handling through Steam.
* Permissions handling for Admin users.
* Category integration.
* Player Profiles.

## Front-end
### Building (Without Docker)
The front-end build can be done with `npm install` in the `/board-portal-2` folder and once dependancies are installed, the client server can be started with `npm start`
#### Features
* Supports querying a running webserver on it's given endpoints for changelog, preview pages, sp maps and coop maps.
* Light and dark theme support.
* Prototyped page designs for many auxiliary pages.

## Original Project Team
This project started as a Senior Capstone Project for the following members.
* [@DanielBatesJ](https://github.com/DanielBatesJ)
* [@MitchellBaker65](https://github.com/MitchellBaker65)
* [@Pixel-Knight](https://github.com/Pixel-Knight)
* [@MurphyMichael](https://github.com/MurphyMichael)
* [@JFiedler23](https://github.com/JFiedler23)

## Project References 
[Original Portal 2 Boards](https://github.com/iVerb1/Portal2Boards)

[Least Portals Github](https://github.com/NeKzor/lp)

