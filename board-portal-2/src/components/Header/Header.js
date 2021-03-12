import { Grid } from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import "./header.css"
import scoreUpdates from "./img/ScoreUpdates.png"
import steamIcon from "./img/steamicon.png"
import singlplayerIcon from "./img/Singleplayer.png"
import fullGameRunsIcon from "./img/running_large.png"
import aggregatedIcon from "./img/Aggregated.jpg"
import coopIcon from "./img/Co-op.png"

const Header = () => {
  return (
    <div className='header'>
      <Grid container>
        <Grid container direction='column'>
          <Grid item className='title'>
            Portal 2 Leaderboards
          </Grid>
          <Grid item className='nav-header'>
            <a href='/' className='nav-links-header'>
              <img src={scoreUpdates} className='img-header' />
              Score Updates
            </a>
            <div to='/home' className='nav-links-header'>
              <img src={singlplayerIcon} className='img-header' />
              Single Player
            </div>
            <div to='/home' className='nav-links-header'>
              <img src={coopIcon} className='img-header' />
              Cooperative
            </div>
            <div to='/home' className='nav-links-header'>
              <img src={aggregatedIcon} className='img-header' />
              Aggregated
            </div>
            <div to='/home' className='nav-links-header'>
              <img src={fullGameRunsIcon} className='img-header' />
              Full Game Runs
            </div>
          </Grid>
        </Grid>
      </Grid>
      <Grid item className='steam'>
        <div className='sepper'></div>
        <div className='steam-text'>Steam</div>
        <img src={steamIcon} style={{ height: "45px" }} />
      </Grid>
    </div>
  )
}

export default Header
