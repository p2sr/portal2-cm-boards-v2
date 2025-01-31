import * as React from 'react';
import { Box, useTheme, Typography } from "@mui/material";
import { tokens } from "../theme";
import LeaderboardIcon from '@mui/icons-material/Leaderboard';
import LeaderboardEntry from './LeaderboardEntry';
import ArrowRightIcon from '@mui/icons-material/ArrowRight';
import ArrowLeftIcon from '@mui/icons-material/ArrowLeft';
import { useState } from 'react';
import { IconButton } from "@mui/material";

const LeaderboardEntries = props => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);

    const [page, setPage] = useState(0);

    var pageEntries = 40;
    var index = page * pageEntries;

    var list = props.data;
    var type = props.type;
    console.log(props)

    if (type == 2) {
        list = list.filter((player) => player[1].num_scores === 108)
        list.sort((a,b) => a[1].score - b[1].score)
    }

    const limitList = list.slice(page * pageEntries, (page + 1) * pageEntries);

    console.log(limitList);

    const leaderboard = limitList.map(entry => {
        index++
        return <Box
        display="flex"
        flexDirection="column"
        style={{
            backgroundColor:
                index % 2 === 0
                ? colors.gray[700]
                : colors.gray[600]
            }}
        >
            <LeaderboardEntry entry={entry} index={index} type={type}/>
        </Box>
    })

    return <div>
        {/* Leaderboard */}
        <Box display="flex" justifyContent="center" padding="15px">
            <Box display="flex"
            justifyContent="center"
            flexDirection="column"
            width="500px"
            style={{borderRadius:"10px"}}
            >
                {/* Title: Leaderboard */}
                <Box
                display="flex"
                padding="10px"
                flexGrow="1"
                backgroundColor={colors.primary[700]}
                style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                alignItems="center"
                >
                    <LeaderboardIcon style={{fontSize:"200%"}}/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="regular"
                        sx={{m : "0 0 0 10px" }}
                        >
                        LEADERBOARD
                    </Typography>
                </Box>

                {/* Scores */}
                <div
                display="flex"
                padding="20px"
                flexGrow="1"
                backgroundColor={colors.primary[600]}
                style={{
                    borderBottomLeftRadius:"10px",
                    backgroundColor:colors.primary[600],
                    borderBottomRightRadius:"10px",
                    width:"100%",
                    padding:"20px",
                    backgroundClip:"padding-box"
                }}
                alignItems="center"
                justifyContent="center"
                >
                    <Box
                    display="flex"
                    padding="10px"
                    flexGrow="1"
                    backgroundColor={colors.primary[700]}
                    style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                    justifyContent="center"
                    alignItems="center">
                        <IconButton disableRipple>
                            <ArrowLeftIcon
                                onClick={() =>setPage(page - 1 < 0 ? page : page - 1)}
                            />
                        </IconButton>
                        <Typography
                            variant="h5"
                            color={colors.gray[100]}
                            fontWeight="regular"
                            >
                            {page} / {Math.floor(list.length / pageEntries)}
                        </Typography>
                        <IconButton disableRipple>
                            <ArrowRightIcon
                                onClick={() => setPage(page + 1 > Math.floor(list.length / pageEntries) ? page : page + 1)}
                            />
                        </IconButton>
                    </Box>
                    {leaderboard}
                </div>
            </Box>
        </Box>
    </div>
}

export default LeaderboardEntries