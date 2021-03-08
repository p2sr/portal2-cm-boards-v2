import { Grid } from "@material-ui/core"
import { BrowserRouter, Link } from "react-router-dom"
import "./header.css"
import scoreUpdates from "./img/ScoreUpdates.png"

const Header = () => {
  return (
    <Grid
      className='header'
      container='true'
      justify='space-between'
      direction='column'>
      <Grid container='true' direction='row'>
        <Grid item='true' className='title'>
          Portal 2 Leaderboards
        </Grid>
        <Grid item='true' className='nav'>
          <div to='/home' className='nav-links'>
            <img src={scoreUpdates} style={{ height: "25px" }} />
            Score Updates
          </div>
          <div to='/home' className='nav-links'>
            Single Player
          </div>
          <div to='/home' className='nav-links'>
            Cooperative
          </div>
          <div to='/home' className='nav-links'>
            Aggregated
          </div>
          <div to='/home' className='nav-links'>
            Full Game Runs
          </div>
        </Grid>
      </Grid>
      <Grid item='true' className='steam'>
        Steam
      </Grid>
    </Grid>
  )
}

export default Header
