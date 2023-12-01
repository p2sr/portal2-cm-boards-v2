import React from 'react'
import YoutubeEmbed from '../scenes/global/YoutubeEmbed'
import { Box, Grid } from '@mui/material'
import { Typography } from "@mui/material"
import { tokens } from "../theme"
import { useTheme } from '@emotion/react'
import DownloadIcon from '@mui/icons-material/Download';
import YouTubeIcon from '@mui/icons-material/YouTube';

const InfoGrid = props => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    return (
        <Grid container spacing={0} flexGrow={props.grow} paddingTop="5px" paddingBottom="5px">
            <Grid item xs={3}>
                <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="Regular"
                    >
                    {props.title}
                </Typography>
            </Grid>
            <Grid item xs={9}>
                <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="Light"
                    >
                    {props.text}
                </Typography>
            </Grid>
        </Grid>
    )
}

const ScoreEntryDropdown = props => {   
    var submission = props.submission
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
  return ( 
    <Box display="flex" padding={"15px"}>
        <Grid container spacing={"20px"}>
            <Grid item xs={3} display="flex">
                <YoutubeEmbed embedId={submission.youtube_id}/>
            </Grid>
            <Grid item xs={5} display="flex">
                <Typography
                    variant="h4"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    >
                    Comment:
                </Typography>
            </Grid>
            <Grid item xs={4} display="flex">
                <Box display="flex" flexDirection="column" style={{width:"100%"}}>
                    <InfoGrid title="COMMENT:" text={submission.note} grow={1}/>
                    <InfoGrid title="SAR Version:" text={submission.note}/>
                    <InfoGrid title="Status:" text={submission.note}/>
                    <Box display="flex" justifyContent="flex-start" paddingTop="5px" gap="10px">
                        <DownloadIcon fontSize='large'/>
                        <YouTubeIcon fontSize='large'/>
                    </Box>
                </Box>
            </Grid>
        </Grid>
    </Box>
  )
}

export default ScoreEntryDropdown