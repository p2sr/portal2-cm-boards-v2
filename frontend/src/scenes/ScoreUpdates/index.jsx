import Topbar from "../global/Topbar";
import React from "react";
import { Box, useTheme, Typography } from "@mui/material";
import { tokens } from "../../theme";
import { useEffect, useState } from "react";
import './index.css';
import PersonIcon from '@mui/icons-material/Person';
import TimelineIcon from '@mui/icons-material/Timeline';
import HistoryIcon from '@mui/icons-material/History';
import ScoreEntries from '../../components/ScoreEntries'

const ENDPOINT = "http://localhost:8080/api/v1/changelog"


const ScoreUpdates = () => {
    const [changelogData, setChangelogData] = useState([])

    //fetching changelog data on first component load
    useEffect(() => {
        const fetchData = async () => {
            let response = await fetch(ENDPOINT)
            return response.json()
        }

        fetchData()
        .then(data => setChangelogData(data))
        .catch(error => console.log(error))
    }, [])

    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    return <div id="main" flexDirection="column" justifyContent="flex-start" style={{"--bgcolor": theme.palette.background.default}}>
        <Topbar />
        {/* Info boxes */}
        <Box display="flex" justifyContent="center">
        <Box display="flex" justifyContent="center" width="93%">
            {/* Title: Following */}
            <Box
            display="flex"
            padding="10px"
            flexGrow="0.25"
            backgroundColor={colors.primary[700]}
            style={{borderRadius:"10px"}}
            alignItems="center"
            sx={{ m: "0 15px 15px 15px" }}
            >
                <PersonIcon style={{fontSize:"200%"}}/>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    sx={{m : "0 0 0 10px" }}
                    >
                    FOLLOWING
                </Typography>
            </Box>
            {/* Title: Daily Activity */}
            <Box
            display="flex"
            padding="10px"
            flexGrow="0.75"
            backgroundColor={colors.primary[700]}
            style={{borderRadius:"10px"}}
            alignItems="center"
            sx={{ m: "0 15px 15px 15px" }}
            >
                <TimelineIcon style={{fontSize:"200%"}}/>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    sx={{m : "0 0 0 10px" }}
                    >
                    DAILY ACTIVITY
                </Typography>
            </Box>
        </Box>
        </Box>

        {/* Score Updates */}
        <Box display="flex" justifyContent="center" padding="15px">
            <Box display="flex"
            justifyContent="center"
            flexDirection="column"
            width="93%"
            style={{borderRadius:"10px"}}
            >
                {/* Title: Score Updates */}
                <Box
                display="flex"
                padding="10px"
                flexGrow="1"
                backgroundColor={colors.primary[700]}
                style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                alignItems="center"
                >
                    <HistoryIcon style={{fontSize:"200%"}}/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="regular"
                        sx={{m : "0 0 0 10px" }}
                        >
                        SCORE UPDATES
                    </Typography>
                </Box>

                {/* Scores */}
                <Box
                display="flex"
                padding="10px"
                flexGrow="1"
                backgroundColor={colors.primary[600]}
                style={{borderBottomLeftRadius:"10px", borderBottomRightRadius:"10px"}}
                alignItems="center"
                justifyContent="center"
                >
                    <ScoreEntries
                        changelogData={changelogData}
                    />
                </Box>
            </Box>
        </Box>
    </div>
}

export default ScoreUpdates;