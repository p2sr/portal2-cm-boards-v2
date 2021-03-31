import { Typography, ButtonBase, Grid, Paper } from "@material-ui/core"
import React from "react"
import { useStyles } from "./style"
import img46362 from "./img/46362.jpg"
import img47455 from "./img/47455.jpg"
import img47738 from "./img/47738.jpg"
import img52759 from "./img/52759.jpg"
import img62759 from "./img/62759.jpg"

const AggregatedSelector = () => {
  const classes = useStyles()
  return (
    <Paper className={classes.bodyPage}>
      <Grid
        container
        direction='column'
        justify='space-around'
        alignItems='center'>
        <Grid item className={classes.cards}>
          <ButtonBase focusRipple className={classes.image}>
            <span
              className={classes.imageSrc}
              style={{
                backgroundImage: `url(${img46362})`
              }}
            />
            <span className={classes.imageBackdrop} />
            <span className={classes.imageButton}>
              <Typography
                component='span'
                variant='subtitle1'
                color='inherit'
                className={classes.imageTitle}>
                Overall
              </Typography>
            </span>
          </ButtonBase>
        </Grid>
        <Grid item className={classes.cards}>
          <ButtonBase focusRipple className={classes.image}>
            <span
              className={classes.imageSrc}
              style={{
                backgroundImage: `url(${img47455})`
              }}
            />
            <span className={classes.imageBackdrop} />
            <span className={classes.imageButton}>
              <Typography
                component='span'
                variant='subtitle1'
                color='inherit'
                className={classes.imageTitle}>
                Single Player
              </Typography>
            </span>
          </ButtonBase>
        </Grid>
        <Grid item className={classes.cards}>
          <ButtonBase focusRipple className={classes.image}>
            <span
              className={classes.imageSrc}
              style={{
                backgroundImage: `url(${img47738})`
              }}
            />
            <span className={classes.imageBackdrop} />
            <span className={classes.imageButton}>
              <Typography
                component='span'
                variant='subtitle1'
                color='inherit'
                className={classes.imageTitle}>
                Cooperative
              </Typography>
            </span>
          </ButtonBase>
        </Grid>
      </Grid>
    </Paper>
  )
}

export default AggregatedSelector
