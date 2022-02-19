# Portal 2 Leaderboards Revision (board.portal2.sr)

A re-write of the original Portal 2 Challenge Mode Leaderboards designed to take advantage of modern web technology and allow for the community to take a more active role in developing the leadboards.

## Should I set up this environment with Docker? Or set it up locally?

[@cesarila](https://github.com/cesarila) has contributed a working Docker solution to building the boards. 
For those unfamiliar with Docker, it allows you to run different container environments that act similar to to lightweigth Virtual Machines, where you can define ahead of time what is installed on the containers when they're built. 
This means minimal technical knowledge needed outside of learning to use Docker (details below). The main drawback of this is that you have less direct control over your environment, and the performance is much worse. 

If you're familiar with development on Linux/WSL, I would recommend installing Postgres, Rust and Node locally, but if you're looking to mess around and learn as a beginner, or only want to work on the Front-end, I would recommend using Docker.
Additionally, if you want to focus on entirely front-end contributions, contact Daniel for information on a publicaly accessible IP you can use to hit the backend API endpoints without needing to build it on your machine.

## Docker Setup Guide (Web-Server & Database)
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
This only needs to be done on changes to entrypoint scripts, dockerfiles, and docker-compose.yaml. This includes changes made by people other than you. 
If you don't want to think about it, always rebuild your containers after pulling from the repo.

### Running
Run containers in the foreground:
* `docker compose up`

Run containers in the background with logging visible:
* `docker compose up &`

Run containers detatched from the current shell (no logging visible):
* `docker compose up -d`

### .env for Docker
Copy the `.env.example` file in the `/server` folder, and rename it to `.env`, change the contents to follow the convention below.
The `DATABASE_URL` field should be identical if the docker files are unchanged, and the `SERVER.HOST` should bind to `0.0.0.0` rather than `127.0.0.1` for running in a Docker container.

```
DATABASE_URL=postgresql://docker:docker@postgres/p2boards
SERVER.HOST=0.0.0.0
SERVER.PORT=8080
PROOF.RESULTS=500
PROOF.DEMO=200
PROOF.VIDEO=200
BACKBLAZE.KEYID=
BACKBLAZE.KEY=
BACKBLAZE.BUCKET=
RUST_LOG=1
RUST_LOG="actix_web=info"
```

### Troubleshooting Database
To rebuild the database volume from the database dump, do the following:
```bash
docker compose down #make sure all containers are stopped
docker volume rm portal2-cm-boards-v2_postgres_data
docker compose build
docker compose up
```
If this doesn't work for any reason, one thing to check is that the file copied in db/Dockerfile is the same as the dump in db/dbdump. If these files don't match, make them match, save your changes, and try the above steps again.

## Local Setup & More Information

## Backend
### Building
The backend binary is currently being re-worked to work with clap, so we build the binary, then run it with arg flags (to be changed). The backend binary calls our webserver for information needed to fetch/upload new scores and calculate points. It does *not* interact with the database on its own.
#### Features
* Pulling Official Single Player Map data from Steam, caching that data to avoid needing to re-parse/compare.
* Supports multithreading with Rayon.
* Queries to the API for comparison data.
* Calculates all point information for the boards
#### Future Plans
The purpose of keeping this backend seperate from the web-server is to off-load some more computationally heavy tasks to an entirely different process for modularity (in theory you should be able to run a cluster of backends). None of this design is final.

## Database
### Building
* Install `postgres` and setup a user (reference the `DATABASE_URL` bellow).
* Open psql console, `CREATE DATABASE p2boards;`
* Load the latest dump from `/db/dbdump` with `psql p2boards < most_recent_dump_file_name.sql`

## Server
### Building
Using sqlx to pass queries to our database. REST API to be documented.

**Be sure to copy the `.env.example` file, remove `.example` from the file name, and change the contents of the file to suite your usecase.**

#### Local .env Example

```
DATABASE_URL=postgresql://danielbatesj:123@localhost/p2boards
SERVER.HOST=0.0.0.0
SERVER.PORT=8080
PROOF.RESULTS=500
PROOF.DEMO=200
PROOF.VIDEO=200
BACKBLAZE.KEYID=
BACKBLAZE.KEY=
BACKBLAZE.BUCKET=
RUST_LOG=1
RUST_LOG="actix_web=info"
```

#### Features:
* Endpoints interacting with the data on the boards.
* Supports db pool and async for non-blocking, fast response to simultanious queries.
#### Future Plans
* Result Caching (redis?).
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

