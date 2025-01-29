import { Typography, useTheme, Grid, IconButton } from "@mui/material"
import { tokens, ranks } from "../theme"
import mapInfo from "./MapInfo"
import { makeStyles } from '@material-ui/styles';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import DropdownHandler from "../scenes/global/DropdownHandler";
import ScoreEntryDropdown from "./ScoreEntryDropdown";
import { timeToText } from "../helpers/time";
import { timeSince } from "../helpers/time";

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

const ScoreEntry = props => {
    const theme = useTheme();
    const classes = useStyles();
    const colors = tokens(theme.palette.mode);
    const { isOpen, toggle } = DropdownHandler(false);
    
    var submission = props.submission

    var mapID = JSON.parse(submission.map_id)
    var time = timeToText(submission.score)
    return (
    <div>
        {/* Data points */}
        <Grid container spacing={0}>
            <Grid item xs={0.75} display="flex" justifyContent="center" alignItems="center" style={{
                backgroundColor: submission.pre_rank === null ? "#00000000" :
                submission.pre_rank > 200 ? ranks[10] : ranks[Math.round((submission.pre_rank)/20)]
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
                backgroundColor: submission.post_rank === null ? "#00000000" :
                submission.post_rank > 200 ? ranks[10] : ranks[Math.round((submission.post_rank)/20)]
            }}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="regular"
                    >
                    {submission.post_rank}
                </Typography>
            </Grid>
            <Grid item xs={2} overflow="hidden" whiteSpace="nowrap" className={classes.customRow}>
                <Typography
                    variant="h6"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    {mapInfo[mapID].title}
                </Typography>
                {/* <Typography
                    variant="h6"
                    color={colors.gray[300]}
                    fontWeight="light"
                    sx={{m : "0 0 0 10px" }}
                    >
                    {mapInfo[mapID].chapter_name}
                </Typography> */}
            </Grid>
            <Grid item xs={2} className={classes.customRow}>
                {/* Player profile picture and name */}
                <img src={submission.avatar} height="100%" alt="P2CM"/>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    {submission.user_name}
                </Typography>
            </Grid>
            <Grid item xs={2} className={classes.customRow}>
                {/* Partner profile picture and name */}
                { submission.coop_id !== null && 
                    <img src={submission.blue_name === submission.user_name ?
                    submission.orange_avatar : submission.blue_avatar}
                    height="100%"
                    alt="P2CM"/>
                }
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    sx={{m : "0 0 0 10px" }}
                    >
                    {submission.coop_id === null ? "" :
                    submission.blue_name === submission.user_name ? submission.orange_name : submission.blue_name}
                </Typography>
            </Grid>
            <Grid item xs={0.75} className={classes.customRowEnd}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="medium"
                    >
                    {time}
                </Typography>
            </Grid>
            <Grid item xs={0.75} className={classes.customRowEnd}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="thin"
                    >
                    {submission.score_delta === null ? "" : "-" + timeToText(submission.score_delta) + "s"}
                </Typography>
            </Grid>
            <Grid item xs={0.75} className={classes.customRowEnd}>
                <Typography
                    variant="h5"
                    color={colors.gray[100]}
                    fontWeight="thin"
                    >
                    {submission.pre_rank === null || submission.post_rank === null ?
                    "" : "-" + (submission.pre_rank - submission.post_rank)}
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
                    {timeSince(submission.timestamp)}
                </Typography>
                {/* <Typography
                    variant="h6"
                    color={colors.gray[100]}
                    fontWeight="light"
                    textAlign="right"
                    >
                    {JSON.stringify(submission.timestamp).slice(1,11)}
                </Typography> */}
            </Grid>
            <Grid item xs={0.75} className={classes.customRowEnd}>
                <IconButton onClick={toggle} disableRipple>
                    <ArrowDropDownIcon sx={{transform: isOpen ? "rotate(90deg)" : "rotate(0deg)"}}/>
                </IconButton>
            </Grid>
        </Grid>
        {isOpen && <ScoreEntryDropdown submission={submission}/>}
    </div>
  )
}

export default ScoreEntry