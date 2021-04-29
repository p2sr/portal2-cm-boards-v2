import Typography from "@material-ui/core/Typography";
import { useStyles } from "./style";
import Grid from "@material-ui/core/Grid";

function Score({ rank, avatar, name, time, score }) {
  const classes = useStyles();

  const getImage = () => {
    if (avatar.length === 1) {
      return (
        <img
          className={classes.scoreImage}
          src={avatar[0]}
          alt="player profile"
        ></img>
      );
    } else if (avatar.length > 1) {
      return (
        <>
          <img
            className={classes.scoreImage}
            src={avatar[0]}
            alt="player profile"
          ></img>
          <img
            className={classes.scoreImage}
            src={avatar[1]}
            alt="player profile"
          ></img>
        </>
      );
    }
  };

  return (
    <Grid
      container
      justify="center"
      alignItems="center"
      className={classes.scoreItem}
    >
      <Grid container alignItems="center" xs={6}>
        <Typography
          className={classes.scoreRank}
          variant="caption"
          component="span"
        >
          {rank}
        </Typography>
        {getImage()}
        <Typography
          className={classes.scoreName}
          variant="body1"
          component="span"
        >
          {name}
        </Typography>
      </Grid>
      <Grid container justify="flex-end" alignItems="center" xs={6}>
        {/*<Typography
        className={classes.scoreTime}
        variant="body1"
        component="span"
      >
        {time}
      </Typography>*/}
        <Typography
          className={classes.scoreText}
          variant="body1"
          component="span"
        >
          {score}
        </Typography>
      </Grid>
    </Grid>
  );
}

export default Score;
