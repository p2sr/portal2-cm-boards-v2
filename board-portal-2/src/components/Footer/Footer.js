import { Grid } from "@material-ui/core"
import "./footer.css"
import donatorsIcon from "./img/cakeicon.png"
import aboutIcon from "./img/cupcake.png"
import discordIcon from "./img/discord_noborder.png"
import githubIcon from "./img/github.png"
import wallOfShameIcon from "./img/wallofshame.png"

const Footer = () => {
  return (
    <div className='footer'>
      <Grid className='Footer' container>
        <Grid item className='nav-footer'>
          <div to='/home' className='nav-links-footer'>
            <img src={donatorsIcon} className='img-footer' />
            Donators
          </div>
          <div to='/home' className='nav-links-footer'>
            <img src={githubIcon} className='img-footer' />
            GitHub
          </div>
          <div to='/home' className='nav-links-footer'>
            <img src={discordIcon} className='img-footer' />
            Discord
          </div>
          <div to='/home' className='nav-links-footer'>
            <img src={wallOfShameIcon} className='img-footer' />
            Wall of Shame
          </div>
          <a href='/about' className='nav-links-footer'>
            <img src={aboutIcon} className='img-footer' />
            About
          </a>
        </Grid>
      </Grid>
    </div>
  )
}

export default Footer
