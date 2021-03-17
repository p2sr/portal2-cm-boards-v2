import { Avatar, Grid, List, ListItem, Typography } from "@material-ui/core"
import { useStyles, CustomButton } from "./style.js"
import React from "react"
import Data from "./example.json"

const Donators = () => {
  const classes = useStyles()

  const loadJSON = () => {
    var obj = JSON.parse(Data)
    for (var k in obj) {
      console.log(k)
    }
  }

  return (
    <div onLoad={loadJSON} id='container' className={classes.bodyWallOfShame}>
      <Typography
        className={classes.titleWallOfShame}
        color='primary'
        variant='h1'>
        Wall Of Shame
      </Typography>
      <Typography
        color='primary'
        variant='subtitle1'
        className={classes.subWallOfShame}>
        {" "}
        Banned players
      </Typography>
      <Grid
        container
        display='flex'
        alignItems='flex-start'
        justify='flex-start'>
        <Grid item className={classes.list}>
          <List>
            <ListItem className={classes.listItem}>
              <Avatar alt='' />
              <Typography variant='body1' color='textPrimary'>
                profile name
              </Typography>
            </ListItem>
          </List>
        </Grid>
      </Grid>
    </div>
  )
}

export default Donators
