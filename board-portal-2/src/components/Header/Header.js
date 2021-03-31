import scoreUpdates from "./img/ScoreUpdates.png"
import steamIcon from "./img/steamicon.png"
import singlplayerIcon from "./img/Singleplayer.png"
import fullGameRunsIcon from "./img/running_large.png"
import aggregatedIcon from "./img/aggregated.png"
import coopIcon from "./img/Co-op.png"
import {
  Grid,
  AppBar,
  Toolbar,
  Typography,
  Button,
  Switch
} from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import { useStyles, CustomButton } from "./style.js"
import Dropdown from "./Dropdown"
import React from "react"
/**
 * @name - Header
 * @desc - Contains the toolbar for header. Displays the website title. Shows links for home page,
 *         single player scores, cooperative scores, aggregated (overall, single player, and cooperative),
 *         full game runs website, and a steam sign in for the website.
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */
const Header = ({ handleChange, themeStatus }) => {
  const classes = useStyles()

  return (
    <div id='container' className={classes.root}>
      <AppBar color='primary' position='static'>
        <Toolbar className={classes.toolbar}>
          <Grid
            container
            justify='space-around'
            alignItems='flex-start'
            direction='column'>
            <Grid item>
              <Typography
                color='textPrimary'
                className={classes.title}
                noWrap
                variant='h5'>
                Portal 2 Leaderboards
              </Typography>
            </Grid>
            <Grid item className={classes.headerLinks}>
              <CustomButton variant='text' href='/'>
                <img src={scoreUpdates} className={classes.icon} />
                Score Updates
              </CustomButton>
              <CustomButton variant='text' href='/sp'>
                <img src={singlplayerIcon} className={classes.icon} />
                Single Player
              </CustomButton>
              <CustomButton variant='text'>
                <img src={coopIcon} className={classes.icon} />
                Cooperative
              </CustomButton>
              <CustomButton variant='text' href='/agg-selector'>
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
          <Grid
            container
            direction='column'
            justify='center'
            alignItems='flex-end'
            xs='2'>
            <Grid item container justify='flex-end'>
              <CustomButton
                className={classes.steam}
                href='https://steamcommunity.com/openid/login?openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0&openid.mode=checkid_setup&openid.return_to=http%3A%2F%2Fboard.iverb.me%2Flogin&openid.realm=http%3A%2F%2Fboard.iverb.me&openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select'
                id='steam-login'>
                <div>Sign In</div>
                <img src={steamIcon} style={{ height: "45px" }} />
              </CustomButton>
            </Grid>
            <Grid item container justify='flex-end' alignItems='center'>
              <Grid item>
                <Typography color='textPrimary'>Dark</Typography>
              </Grid>
              <Switch
                color='default'
                checked={themeStatus}
                onChange={handleChange}
              />
              <Grid>
                <Typography color='textPrimary'>Light</Typography>
              </Grid>
            </Grid>
          </Grid>
        </Toolbar>
      </AppBar>
    </div>
  )
}

export default Header
