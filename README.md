# portal-2-boards-capstone-CSCE4901

## Group Memebers
* Daniel Bates   -> @DanielBatesJ
* Mitchell Baker -> @MitchellBaker65
* Josh Bednarz   -> @Pixel-Knight
* Michael Murphy -> @MurphyMichael
* John Fiedler   -> @JFiedler23

## Building

### Backend
The backend binary can be build by using `cargo build` in the `/backend` directory. With Rust installed, it should download all dependancies and compile the binary for you.
Current features from running this binary:
* Will create all SP map JSON information in `/server/api/maps/sp`
* Planned for the future is support for specifying the functions with environment variables (such as adding new times, updating specific maps etc)

### Server
The server can be started with `cargo run` in the `server` directory in much the same way as the backend. This should keep a server running on the terminal instance you ran it on.

Be sure to copy the `.env.example` file, remove `.example` from the file name, and change the contents of the file to suite your usecase.

Current Features:
* Listens on the specified port for your host for GET requests for sp maps. The endpoint is `/api/maps/sp/{mapid}` where the mapid is the Steamleaderboard ID for the singleplayer map.

### Front-end

The front-end build can be done with `npm install` in the `/board-portal-2` folder and once dependancies are installed, the client server can be started with `npm start`

## Project Specs
[Software Requirements Specifications](https://docs.google.com/document/d/1HnGGvk6OQsIHrAGKlBZczCmHcyulmhzpf-vH-jAk1AU/edit?usp=sharing)


## Project References 
[Original Portal 2 Boards](https://github.com/iVerb1/Portal2Boards)

[Least Portals Github](https://github.com/NeKzor/lp)

