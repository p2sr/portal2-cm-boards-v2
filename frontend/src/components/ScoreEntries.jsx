import { Box, Typography, useTheme } from "@mui/material"
import { tokens } from "../theme"
import { useEffect, useState } from "react";
import mapInfo from "./MapInfo"
import YouTubeIcon from '@mui/icons-material/YouTube';
import DownloadIcon from '@mui/icons-material/Download';
import ChatBubbleIcon from '@mui/icons-material/ChatBubble';

const ENDPOINT = "http://localhost:8080/api/v1/coop_bundles"

const ScoreEntries = props => {
    const theme = useTheme();
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
                console.log(props)
                var mapID = JSON.parse(submission.map_id)
                var time = JSON.stringify(submission.score)

                if (time.length > 4) {
                    time = time.slice(0, time.length - 4) + ":" + time.slice(time.length - 4, time.length - 2) + "." + time.slice(time.length - 2)
                }
                else {
                    time = time.slice(0, time.length - 2) + "." + time.slice(time.length - 2)
                }
                index++

                return <Box
                display="flex"
                style={{
                    backgroundColor:
                      index % 2 === 0
                        ? colors.gray[600]
                        : colors.gray[500]
                  }}
                >
                    {/* Map image and name */}
                    <Box display="flex" alignItems="center" width="400px">
                        <img src={mapInfo[mapID].image} height="40px" alt="P2CM"/>
                        <Typography
                            variant="h4"
                            color={colors.gray[100]}
                            fontWeight="regular"
                            sx={{m : "0 0 0 10px" }}
                            >
                            {mapInfo[mapID].title}
                        </Typography>
                    </Box>

                    {/* Player profile picture and name */}
                    <Box display="flex" alignItems="center" width="250px">
                        <img src={submission.avatar} width="30px" alt="P2CM"/>
                        <Typography
                            variant="h5"
                            color={colors.gray[100]}
                            fontWeight="regular"
                            sx={{m : "0 0 0 10px" }}
                            >
                            {submission.user_name}
                        </Typography>
                    </Box>

                    {/* Partner profile picture and name */}
                    <Box display="flex" alignItems="center" width="250px">
                        <img src={submission.avatar} width="30px" alt="P2CM"/>
                        <Typography
                            variant="h5"
                            color={colors.gray[100]}
                            fontWeight="regular"
                            sx={{m : "0 0 0 10px" }}
                            >
                            {submission.user_name}
                        </Typography>
                    </Box>

                    {/* Data points */}
                    <Box display="flex">
                        <Box display="flex" width="100px" style={{backgroundColor:"#8296c9"}} height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.pre_rank}
                            </Typography>
                        </Box>
                        <Box display="flex" width="112px" style={{backgroundColor:"#7dba76"}} height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.post_rank}
                            </Typography>
                        </Box>
                        <Box display="flex" width="112px" height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {submission.score_delta}
                            </Typography>
                        </Box>
                        <Box display="flex" width="112px" height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h6"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {time}
                            </Typography>
                        </Box>
                        <Box display="flex" width="112px" height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h6"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {timeSince(submission.timestamp)}
                            </Typography>
                        </Box>
                        <Box display="flex" width="112px" height="100%" alignItems="center" justifyContent="center">
                            <Typography
                                variant="h6"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                >
                                {JSON.stringify(submission.timestamp).slice(1,11)}
                            </Typography>
                        </Box>
                        <Box display="flex" alignItems="center" padding="10px">
                            {submission.note && (
                                <ChatBubbleIcon />
                            )}
                        </Box>
                        <Box display="flex" alignItems="center" padding="10px">
                            {submission.has_demo && (
                                <DownloadIcon />
                            )}
                        </Box>
                        <Box display="flex" alignItems="center" padding="10px">
                            {submission.youtube_id && (
                                <YouTubeIcon />
                            )}
                        </Box>
                    </Box>
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

    console.log(second_diff)
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

export default ScoreEntries