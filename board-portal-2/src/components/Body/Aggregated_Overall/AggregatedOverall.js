import {
  Box,
  Button,
  Grid,
  IconButton,
  Paper,
  Typography
} from "@material-ui/core"
import { ArrowBackIos, ArrowForwardIos } from "@material-ui/icons"
import React, { useContext } from "react"
import { CustomIconButton, useStyles } from "./style"
import imgOverall from "./img/overall.jpg"
import ScoreList from "./ScoreList"
import TimeList from "./TimeList"
import { ThemeContext } from "../../../App.js"

const AggregatedOverall = () => {
  const classes = useStyles()
  const themeContext = useContext(ThemeContext)
  return (
    <Paper className={classes.root}>
      <Grid container direction='row' className={classes.image}>
        <span
          className={classes.imageSrc}
          style={{
            backgroundImage: `url(${imgOverall})`
          }}
        />
        <Grid container justify='center' alignItems='center' direction='column'>
          <Typography
            color='inherit'
            variant='h4'
            className={classes.imageSubTitle}>
            Aggregated Points and Times
          </Typography>
          <Typography
            color='inherit'
            variant='h4'
            className={classes.imageTitle}>
            Overall
          </Typography>
        </Grid>
        <Grid
          container
          direction='row'
          alignItems='flex-end'
          className={classes.showingSelector}>
          <Grid
            container
            justify='flex-start'
            alignItems='center'
            style={{ width: "50%" }}>
            <CustomIconButton>
              <ArrowBackIos />
            </CustomIconButton>
            <Typography>page# / page#</Typography>
            <CustomIconButton>
              <ArrowForwardIos />
            </CustomIconButton>
          </Grid>
          <Grid
            container
            justify='flex-end'
            alignItems='center'
            style={{ width: "50%" }}>
            <CustomIconButton>
              <ArrowBackIos />
            </CustomIconButton>
            <Typography>page# / page#</Typography>
            <CustomIconButton>
              <ArrowForwardIos />
            </CustomIconButton>
          </Grid>
        </Grid>
      </Grid>
      <Grid container>
        <Grid container className={classes.lists} style={{ width: "50%" }}>
          <ScoreList themeStatus={themeContext.themeStatus} />
        </Grid>
        <Grid container className={classes.lists} style={{ width: "50%" }}>
          <TimeList themeStatus={themeContext.themeStatus} />
        </Grid>
      </Grid>
    </Paper>
  )
}

export default AggregatedOverall
