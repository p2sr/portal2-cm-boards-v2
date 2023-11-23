import { Box, Typography, useTheme, Grid, Icon } from "@mui/material"
import { tokens } from "../theme"
import { useEffect, useState } from "react";
import mapInfo from "./MapInfo"
import { makeStyles } from '@material-ui/styles';
import YouTubeIcon from '@mui/icons-material/YouTube';
import DownloadIcon from '@mui/icons-material/Download';
import ChatBubbleIcon from '@mui/icons-material/ChatBubble';

const ENDPOINT = "http://localhost:8080/api/v1/coop"

const useStyles = makeStyles((theme) => ({
    customRow: {
      height: 46, // Set your desired height here
      display: 'flex',
      justifyContent: 'flex-start',
      alignItems: 'center',
    },
    customRowEnd: {
        height: 46, // Set your desired height here
        display: 'flex',
        justifyContent: 'flex-end',
        alignItems: 'center',
      },
}));

const ScoreEntries = props => {
    const theme = useTheme();
    const classes = useStyles();
    const colors = tokens(theme.palette.mode);
    const [coopData, setCoopData] = useState([])

    var index = 0

        //fetching changelog data on first component load
        useEffect(() => {
            const fetchData = async () => {
                let response = await fetch(ENDPOINT)
                return response.json()
            }
    
            fetchData()
            .then(data => setCoopData(data))
            .catch(error => console.log(error))
        }, [])

    return <div flexDirection="column" justifyContent="flex-start">
        {
            props.changelogData.map(submission => {
                var mapID = JSON.parse(submission.map_id)
                var time = timeToText(submission.score)

                index++
                return <Box
                display="flex"
                style={{
                    backgroundColor:
                      index % 2 === 0
                        ? colors.gray[700]
                        : colors.gray[600]
                  }}
                >
                    {/* Data points */}
                    <Grid container spacing={0}>
                        <Grid item xs={0.75} display="flex" justifyContent="center" alignItems="center" style={{
                            backgroundColor:'rgba(20, 180, 10, 0.3)'
                        }}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.pre_rank}
                            </Typography>
                        </Grid>
                        <Grid item xs={0.75} display="flex" justifyContent="center" alignItems="center" style={{
                            backgroundColor:'rgba(100, 20, 208, 0.3)'
                        }}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.post_rank}
                            </Typography>
                        </Grid>
                        <Grid item xs={2} overflow="hidden" whiteSpace="nowrap">
                            <Typography
                                variant="h6"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                sx={{m : "0 0 0 10px" }}
                                >
                                {mapInfo[mapID].title}
                            </Typography>
                            <Typography
                                variant="h6"
                                color={colors.gray[300]}
                                fontWeight="light"
                                sx={{m : "0 0 0 10px" }}
                                >
                                {mapInfo[mapID].chapter_name}
                            </Typography>
                        </Grid>
                        <Grid item xs={2} className={classes.customRow}>
                            {/* Player profile picture and name */}
                            <img src={submission.avatar} height="100%" alt="P2CM"/>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                sx={{m : "0 0 0 10px" }}
                                >
                                {submission.user_name}
                            </Typography>
                        </Grid>
                        <Grid item xs={2} className={classes.customRow}>
                            {/* Partner profile picture and name */}
                            <img src={submission.avatar} height="100%" alt="P2CM"/>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                sx={{m : "0 0 0 10px" }}
                                >
                                {console.log(coopData[1][0].user_name1)}
                            </Typography>
                        </Grid>
                        <Grid item xs={0.75} className={classes.customRowEnd}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {time}
                            </Typography>
                        </Grid>
                        <Grid item xs={0.75} className={classes.customRowEnd}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.score_delta === null ? "" : "-" + timeToText(submission.score_delta) + "s"}
                            </Typography>
                        </Grid>
                        <Grid item xs={0.75} className={classes.customRowEnd}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {time}
                            </Typography>
                        </Grid>
                        <Grid item xs={1.5}>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                textAlign="right"
                                >
                                {timeSince(submission.timestamp)}
                            </Typography>
                            <Typography
                                variant="h6"
                                color={colors.gray[100]}
                                fontWeight="light"
                                textAlign="right"
                                >
                                {JSON.stringify(submission.timestamp).slice(1,11)}
                            </Typography>
                        </Grid>
                        <Grid item xs={0.75} display="flex" alignItems="center" justifyContent="center">
                            <YouTubeIcon/>
                        </Grid>
                    </Grid>
                </Box>
            })
        }
    </div>
}

function timeSince (timestamp) {
    let current_date = new Date()
    let previous_date = new Date(timestamp)
    let second_diff = ((current_date.getTime() - previous_date.getTime()) / 1000)
    let message = "a"

    if (second_diff < 60) {
        message = second_diff + " seconds ago";
    } else if (second_diff < 3600) {
        message = Math.round(second_diff / 60) + " minutes ago";
    } else if (second_diff < 86400) {
        message = Math.round(second_diff / 3600) + " hours ago";
    } else if (second_diff < 2620800) {
        message = Math.round(second_diff / 86400) + " days ago";
    } else if (second_diff < 31449600) {
        message = Math.round(second_diff / 2620800) + " months ago";
    } else {
        message = Math.round(second_diff / 31449600) + " years ago";
    }
    return message
}

function timeToText (timeBefore) {
    if (timeBefore != null) {
        var time = JSON.stringify(timeBefore)
        time = time.replace("-","")

        if (time.length > 4) {
            time = time.slice(0, time.length - 4) + ":" + time.slice(time.length - 4, time.length - 2) + "." + time.slice(time.length - 2)
        } else if (time.length > 2){
            time = time.slice(0, time.length - 2) + "." + time.slice(time.length - 2)
        } else if (time.length === 2){
            time = "0." + time
        } else {
            time = "0.0" + time
        }
    }
    return time
}

export default ScoreEntries