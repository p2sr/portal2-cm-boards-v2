import { Box, IconButton, Typography, useTheme} from "@mui/material";
import { useContext } from "react";
import { ColorModeContext, tokens } from "../../theme";
import LightModeOutlinedIcon from "@mui/icons-material/LightModeOutlined";
import DarkModeOutlinedIcon from "@mui/icons-material/LightModeOutlined";
import MoreHorizIcon from '@mui/icons-material/MoreHoriz';
import LocationOnIcon from '@mui/icons-material/LocationOn';
import HistoryIcon from '@mui/icons-material/History';
import LeaderboardIcon from '@mui/icons-material/Leaderboard';
import Logo from './img/portal2logo.png';
import PFP from './img/pfp.png';

const Topbar = () => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    const colorMode = useContext(ColorModeContext);
    return (
    <Box display="flex" justifyContent="center" p={2} gap="100px">

        {/* LOGO */}
        <Box display="flex">
            <Box display="flex" alignItems="center">
                <IconButton disableRipple>
                    <img src={Logo} style={{width:"25%"}} alt="P2CM"/>
                    <Typography
                        variant="h1"
                        color={colors.gray[100]}
                        fontWeight="medium" 
                        sx={{m : "0 0 0 10px" }}
                        >
                        P2CM
                    </Typography>
                </IconButton>
            </Box>
        </Box>

        {/* TABS */}
        <Box display="flex">
            <Box
            display="flex"
            alignItems="center"
            gap="20px"
            >
                <IconButton disableRipple>
                    <HistoryIcon />
                    <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    SCORE UPDATES
                    </Typography>
                </IconButton>
                <IconButton disableRipple>
                    <LocationOnIcon/>
                    <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    MAP LIST
                    </Typography>
                </IconButton>
                <IconButton disableRipple>
                    <LeaderboardIcon/>
                    <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    LEADERBOARD
                    </Typography>
                </IconButton>
                <IconButton disableRipple>
                    <MoreHorizIcon/>
                    <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    MORE
                    </Typography>
                </IconButton>
            </Box>
        </Box>

        {/* PROFILE */}
        <Box
        display="flex"
        alignItems="center"
        >
            <IconButton disableRipple>
                <img src={PFP}
                width="50px"
                height="50px"
                style={{borderRadius:"10px"}}
                alt="Profile"
                />
            </IconButton>
            <IconButton onClick={colorMode.toggleColorMode} disableTouchRipple>
                {theme.palette.mode === 'dark' ? (
                    <DarkModeOutlinedIcon/>
                ) : (
                    <LightModeOutlinedIcon/>
                )}
            </IconButton>
        </Box>
    </Box>)
}

export default Topbar;