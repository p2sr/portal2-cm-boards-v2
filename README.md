# Portal 2 Leaderboards Revision (board.portal2.sr)

## Docker (Backend and Database)
### Prerequisites
1. Docker
    1. https://docs.docker.com/get-docker/
    1. Optional Post-Installation Steps:
        - Linux: https://docs.docker.com/engine/install/linux-postinstall/
1. Have docker-compose available (Linux only, comes bundled with Docker desktop for Mac and Windows):
    - https://docs.docker.com/compose/cli-command/#install-on-linux
    - Short Version: 
        ```bash
        mkdir -p ~/.docker/cli-plugins/
        sudo curl -L "https://github.com/docker/compose/releases/download/v2.1.0/docker-compose-$(uname -s)-$(uname -m)" -o ~/.docker/cli-plugins/docker-compose
        sudo chmod +x ~/.docker/cli-plugins
        ```
        - Note: v2.1.0 is the latest release at the time of writing this. All releases visible at: https://github.com/docker/compose/releases/
    - Shortest Version: https://gist.github.com/cesarila/a58504fbc3924bcef6ae8ce83a652071

### Building
Assuming you've satisfied the prerequisites, you can build the containers from the root directory of this repo with:
`docker compose build`
This only needs to be done on changes to entrypoint scripts, dockerfiles, and docker-compose.yaml. This includes changes made by people other than you. If you don't want to think about it, always rebuild your containers after pulling from the repo.

### Running
Run containers in the foreground:
`docker compose up`
Run containers in the background with logging visible:
`docker compose up &`
Run containers detatched from the current shell (no logging visible):
`docker compose up -d`

### Troubleshooting Database
To rebuild the database volume from the database dump, do the following:
```bash
docker compose down #make sure all containers are stopped
docker volume rm portal2-cm-boards-v2_postgres_data
docker compose build
docker compose up
```
If this doesn't work for any reason, one thing to check is that the file copied in db/Dockerfile is the same as the dump in db/dbdump. If these files don't match, make them match, save your changes, and try the above steps again.
## Backend
### Building
The backend binary can be build by using `cargo build` in the `backend` directory. With Rust installed, it should download all dependancies and compile the binary for you.
#### Features
* Pulling Official Single Player Map data from Steam, caching that data to avoid needing to re-parse/compare.
* Supports multithreading with Rayon.
* Queries to the API for comparison data.
#### Future Plans
Plans for the future include allowing args for the binary to allow it to distinguish between different tasks, but the current goal is to use it for fetching / process / cache times from the 
Steam Leaderboards, as well as to calculate and cache profile data for all players.

The purpose of keeping this backend seperate from the web-server is to off-load some more computationally heavy tasks to an entirely different process for modularity. None of this design is final

## Database
### Building
* Install `postgres` and setup a user (reference the `DATABASE_URL` bellow.
* Open psql console, `CREATE DATABASE p2boards;`
* Load the latest dump from `/db/dbdump` with `psql p2boards < most_recent_dump_file_name.sql`

## Server
### Building
The code is being re-writen to no longer use Diesel.rs and MySQL. More information to come.

**Be sure to copy the `.env.example` file, remove `.example` from the file name, and change the contents of the file to suite your usecase.**

#### .env Example

```
DATABASE_URL=postgresql://username:password@postgres:5432/p2boards
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
### Building
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

