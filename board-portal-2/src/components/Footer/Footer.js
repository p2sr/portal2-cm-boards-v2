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
            <img
              src={donatorsIcon}
              style={{
                height: "22px",
                paddingLeft: ".5em",
                paddingRight: ".5em"
              }}
            />
            Donators
          </div>
          <div to='/home' className='nav-links-footer'>
            <img
              src={githubIcon}
              style={{
                height: "22px",
                paddingLeft: ".5em",
                paddingRight: ".5em"
              }}
            />
            GitHub
          </div>
          <div to='/home' className='nav-links-footer'>
            <img
              src={discordIcon}
              style={{
                height: "22px",
                paddingLeft: ".5em",
                paddingRight: ".5em"
              }}
            />
            Discord
          </div>
          <div to='/home' className='nav-links-footer'>
            <img
              src={wallOfShameIcon}
              style={{
                height: "22px",
                paddingLeft: ".5em",
                paddingRight: ".5em"
              }}
            />
            Wall of Shame
          </div>
          <div to='/home' className='nav-links-footer'>
            <img
              src={aboutIcon}
              style={{
                height: "22px",
                paddingLeft: ".5em",
                paddingRight: ".5em"
              }}
            />
            About
          </div>
        </Grid>
      </Grid>
    </div>
  )
}

export default Footer
