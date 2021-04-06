import {
  Avatar,
  Grid,
  List,
  ListItem,
  Typography,
  Box
} from "@material-ui/core"
import { useStyles, CustomButton } from "./style.js"
import React from "react"
import Data from "./example.json"

/**
 * @name - WallOfShame
 * @desc - Contains the page displaying the banned players from the sight.
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

const Donators = () => {
  const classes = useStyles()

  return (
    <Box id='container' className={classes.bodyWallOfShame}>
      <Typography
        className={classes.titleWallOfShame}
        color='textPrimary'
        variant='h1'>
        Wall Of Shame
      </Typography>
      <Typography
        color='textSecondary'
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
    </Box>
  )
}

export default Donators
