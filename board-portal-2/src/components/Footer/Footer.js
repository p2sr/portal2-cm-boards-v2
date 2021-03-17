import donatorsIcon from "./img/cakeicon.png"
import aboutIcon from "./img/cupcake.png"
import discordIcon from "./img/discord_noborder.png"
import githubIcon from "./img/github.png"
import wallOfShameIcon from "./img/wallofshame.png"
import { Grid, AppBar, Toolbar, Typography, Button } from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import { useStyles, CustomButton } from "./style.js"

/**
 * @name - Footer
 * @desc - Footer toolbar that contains links to the community Discord,
 *         site GitHub, Wall of Shame, Donators, and About page.
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

const Footer = () => {
  const classes = useStyles()
  return (
    <div id='container-footer' className='container' className={classes.root}>
      <AppBar color='secondary' position='static'>
        <Toolbar className={classes.toolbar}>
          <Grid container justify='space-around' alignContent='center'>
            <BrowserRouter>
              <CustomButton href='/donators'>
                <img src={donatorsIcon} className={classes.icon} />
                Donators
              </CustomButton>
              <CustomButton
                href='https://github.com/DanielBatesJ/portal-2-boards-capstone'
                target='_blank'>
                <img src={githubIcon} className={classes.icon} />
                GitHub
              </CustomButton>
              <CustomButton
                href='https://discordapp.com/invite/hRwE4Zr'
                target='_blank'>
                <img src={discordIcon} className={classes.icon} />
                Discord
              </CustomButton>
              <CustomButton href='/wall_of_shame'>
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
