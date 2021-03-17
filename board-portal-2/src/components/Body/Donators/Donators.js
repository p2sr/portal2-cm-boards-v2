import { Avatar, Grid, List, ListItem, Typography } from "@material-ui/core"
import { useStyles, CustomButton } from "./style.js"
import donatorImg from "./img/portal_2_birthday_by_curtisru-d47w3js.png"
import Data from "./example.json"
import React from "react"

/**
 * @name - Donators
 * @desc - List of all of those who donated to the community in descending order.
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

const Donators = () => {
  const classes = useStyles()

  const loadJSON = () => {
    var obj = JSON.parse(Data)
    for (var k in obj) {
      console.log(k)
    }
  }

  return (
    <div onLoad={loadJSON} id='container' className={classes.bodyDonators}>
      <Typography
        className={classes.titleDonators}
        color='primary'
        variant='h1'>
        Donators
      </Typography>
      <Grid
        container
        display='flex'
        alignItems='flex-start'
        justify='space-between'>
        <Grid item style={{ width: "40%" }}>
          <List>
            <ListItem className={classes.listItem}>
              <Avatar alt='' />
              <Typography variant='body1' color='textPrimary'>
                profile name
              </Typography>
              <Typography color='textPrimary'>$10000</Typography>
            </ListItem>
          </List>
        </Grid>
        <img src={donatorImg} className={classes.img} />
      </Grid>
    </div>
  )
}

export default Donators
