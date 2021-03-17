import scoreUpdates from "./img/ScoreUpdates.png"
import steamIcon from "./img/steamicon.png"
import singlplayerIcon from "./img/Singleplayer.png"
import fullGameRunsIcon from "./img/running_large.png"
import aggregatedIcon from "./img/aggregated.png"
import coopIcon from "./img/Co-op.png"
import { Grid, AppBar, Toolbar, Typography, Button } from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import { useStyles, CustomButton } from "./style.js"
import Dropdown from "./Dropdown"

const Header = () => {
  const classes = useStyles()
  return (
    <div id='container' className={classes.root}>
      <AppBar position='static'>
        <Toolbar className={classes.toolbar}>
          <Grid
            container
            justify='space-around'
            alignItems='flex-start'
            direction='column'>
            <Grid item>
              <Typography className={classes.title} noWrap variant='h5'>
                Portal 2 Leaderboards
              </Typography>
            </Grid>
            <Grid item className={classes.headerLinks}>
              <CustomButton variant='text' href='/'>
                <img src={scoreUpdates} className={classes.icon} />
                Score Updates
              </CustomButton>
              <CustomButton variant='text'>
                <img src={singlplayerIcon} className={classes.icon} />
                Single Player
              </CustomButton>
              <CustomButton variant='text'>
                <img src={coopIcon} className={classes.icon} />
                Cooperative
              </CustomButton>
              <CustomButton variant='text'>
                <img src={aggregatedIcon} className={classes.icon} />
                Aggregated
              </CustomButton>
              {/* <Dropdown /> */}
              <CustomButton
                variant='text'
                href='https://www.speedrun.com/Portal_2'>
                <img src={fullGameRunsIcon} className={classes.icon} />
                Full Game Runs
              </CustomButton>
            </Grid>
          </Grid>
          <CustomButton className={classes.steam}>
            <div>Sign In</div>
            <img src={steamIcon} style={{ height: "45px" }} />
          </CustomButton>
        </Toolbar>
      </AppBar>
    </div>
  )
}

export default Header
