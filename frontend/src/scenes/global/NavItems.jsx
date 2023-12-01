import MoreHorizIcon from '@mui/icons-material/MoreHoriz';
import LocationOnIcon from '@mui/icons-material/LocationOn';
import HistoryIcon from '@mui/icons-material/History';
import LeaderboardIcon from '@mui/icons-material/Leaderboard';
import "./TopbarItem.css"

export const leaderboardDropdown = [
    {
        id: 1,
        title: "Overall",
        path: "./leaderboard/overall",
        cName: "submenu-item",
    },
    {
        id: 2,
        title: "Singleplayer",
        path: "./leaderboard/sp",
        cName: "submenu-item",
    },
    {
        id: 3,
        title: "Cooperative",
        path: "./leaderboard/coop",
        cName: "submenu-item",
    },
]

export const moreDropdown = [
    {
        id: 1,
        title: "Full Game Runs",
        path: "./more/fullgame",
        cName: "submenu-item",
    },
    {
        id: 2,
        title: "Discord",
        path: "./more/discord",
        cName: "submenu-item",
    },
    {
        id: 3,
        title: "GitHub",
        path: "./more/github",
        cName: "submenu-item",
    },
    {
        id: 4,
        title: "Donators",
        path: "./more/donators",
        cName: "submenu-item",
    },
    {
        id: 5,
        title: "Wall of Shame",
        path: "./more/wos",
        cName: "submenu-item",
    },
    {
        id: 6,
        title: "About",
        path: "./more/about",
        cName: "submenu-item",
    },
]

export const topbarItems = [
    {
        id: 1,
        title: "SCORE UPDATES",
        path: "./",
        icon: <HistoryIcon/>,
        dropdown: null
    },
    {
        id: 2,
        title: "MAP LIST",
        path: "./maps",
        icon: <LocationOnIcon/>,
        dropdown: null
    },
    {
        id: 2,
        title: "LEADERBOARD",
        path: "./leaderboard/overall",
        icon: <LeaderboardIcon/>,
        dropdown: leaderboardDropdown
    },
    {
        id: 3,
        title: "MORE",
        path: "./more",
        icon: <MoreHorizIcon/>,
        dropdown: moreDropdown
    },
]