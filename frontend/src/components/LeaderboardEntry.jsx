import { Typography, useTheme, Grid } from "@mui/material"
import { tokens } from "../theme"
import { makeStyles } from '@material-ui/styles';
import { scoreToText } from "../helpers/time";
import emptyFlag from "../scenes/global/img/emptyflag.png";
const flagLink = "https://upload.wikimedia.org/wikipedia/en/thumb/4/4c/Flag_of_Sweden.svg/800px-Flag_of_Sweden.svg.png"

const useStyles = makeStyles((theme) => ({
    customRow: {
      height: 40, // Set your desired height here
      display: 'flex',
      justifyContent: 'flex-start',
      alignItems: 'center',
    },
    customRowCenter: {
        height: 40, // Set your desired height here
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      },
    customRowEnd: {
        height: 40, // Set your desired height here
        display: 'flex',
        justifyContent: 'flex-end',
        alignItems: 'center',
      },
}));

const LeaderboardEntry = props => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    const classes = useStyles();

    const entry = props.entry[1]
    console.log(props)
    const score = props.type == 1 ? Math.round(entry.points) : scoreToText(entry.score)
    const flag = entry.nationality == null ? flagLink : entry.nationality;
    return (
        <div>
            {/* Data points */}
            
            <Grid container spacing={0}>    
                <Grid item xs={2.25} className={classes.customRowCenter}>
                    {/* Player profile picture and name */}
                    <img src={entry.avatar} height="100%" alt="P2CM"/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="medium"
                        sx={{m : "0 0 0 0" }}
                        width="100%"
                        align="center"
                        >
                        {props.index} {/* Rank */}
                    </Typography>
                </Grid>
                <Grid item xs={4.75} className={classes.customRow}>
                    <img src={emptyFlag} height="50%" alt="P2CM" style={{borderRadius: 3}}/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="medium"
                        sx={{m : "0 0 0 12px" }}
                        >
                        {entry.user_name}
                    </Typography>
                </Grid>
                <Grid item xs={5} className={classes.customRowEnd}>
                <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="medium"
                        sx={{m : "0 10px 0 0" }}
                        >
                        {score}
                    </Typography>
                </Grid>
            </Grid>
        </div>
        )
}

export default LeaderboardEntry