import { Typography, useTheme, Grid, IconButton } from "@mui/material"
import { tokens, ranks } from "../theme"
import mapInfo from "./MapInfo"
import { makeStyles } from '@material-ui/styles';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import DropdownHandler from "../scenes/global/DropdownHandler";
import ScoreEntryDropdown from "./ScoreEntryDropdown";
import { timeSince, timeToText } from "../helpers/time";
import ChatBubbleIcon from '@mui/icons-material/ChatBubble';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import YouTubeIcon from '@mui/icons-material/YouTube';
import DownloadIcon from '@mui/icons-material/Download';

const useStyles = makeStyles((theme) => ({
    customRow: {
      height: 40, // Set your desired height here
      display: 'flex',
      justifyContent: 'flex-start',
      alignItems: 'center',
    },
    customRowEnd: {
        height: 40, // Set your desired height here
        display: 'flex',
        justifyContent: 'flex-end',
        alignItems: 'center',
      },
}));

const SPMapScoreEntry = props => {
    const theme = useTheme();
    const classes = useStyles();
    const colors = tokens(theme.palette.mode);
    const { isOpen, toggle } = DropdownHandler(false);
    

    const mapData = props.submission.map_data
    const rank = props.submission.rank
    const points = props.submission.points

    return (
    <div>
        {/* Data points */}
        <Grid container spacing={0}>
            <Grid item xs={0.5} display="flex" justifyContent="center" alignItems="center">
                <Typography
                    variant="h6"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 0px" }}
                    >
                    {rank}
                </Typography>
            </Grid>
            <Grid item xs={6.5} className={classes.customRow}>
                {/* Player profile picture and name */}
                <img src={mapData.avatar} height="100%" alt="P2CM"/>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    {mapData.user_name}
                </Typography>
            </Grid>
            <Grid item xs={1.5} className={classes.customRowEnd}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    textAlign="right"
                    marginRight="5px"
                    >
                    {mapData.timestamp != null ? timeSince(mapData.timestamp) : ""}
                </Typography>
            </Grid>
            <Grid item xs={1.5} className={classes.customRowEnd}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    textAlign="right"
                    marginRight="5px"
                    >
                    {timeToText(mapData.score)}
                </Typography>
            </Grid>
            <Grid item xs={2} overflow="hidden" whiteSpace="nowrap" gap="5px" className={classes.customRowEnd}>
                <ChatBubbleIcon/>
                <PlayArrowIcon/>
                <YouTubeIcon/>
                <DownloadIcon/>
            </Grid>
        </Grid>
    </div>
  )
}

export default SPMapScoreEntry