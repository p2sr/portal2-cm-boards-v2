import donatorsIcon from "./img/cakeicon.png"
import aboutIcon from "./img/cupcake.png"
import discordIcon from "./img/discord_noborder.png"
import githubIcon from "./img/github.png"
import wallOfShameIcon from "./img/wallofshame.png"
import { Grid, AppBar, Toolbar, Typography, Button } from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import { useStyles, CustomButton } from "./style.js"

const Footer = () => {
  const classes = useStyles()
  return (
    <div id='container-footer' className='container' className={classes.root}>
      <AppBar color='secondary' position='static'>
        <Toolbar className={classes.toolbar}>
          <Grid container justify='space-around' alignContent='center'>
            <BrowserRouter>
              <CustomButton to='/home'>
                <img src={donatorsIcon} className={classes.icon} />
                Donators
              </CustomButton>
              <CustomButton to='/home'>
                <img src={githubIcon} className={classes.icon} />
                GitHub
              </CustomButton>
              <CustomButton to='/home'>
                <img src={discordIcon} className={classes.icon} />
                Discord
              </CustomButton>
              <CustomButton to='/home'>
                <img src={wallOfShameIcon} className={classes.icon} />
                Wall of Shame
              </CustomButton>
              <CustomButton href='/about'>
                <img src={aboutIcon} className={classes.icon} />
                About
              </CustomButton>
            </BrowserRouter>
          </Grid>
        </Toolbar>
      </AppBar>
    </div>
  )
}

export default Footer
