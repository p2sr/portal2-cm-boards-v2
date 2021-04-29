import { useStyles } from "./style";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";

function Header({ mapName, chapterName, image }) {
  const classes = useStyles();

  return (
    <Grid
      className={classes.header}
      container
      direction="column"
      alignItems="center"
      justify="center"
    >
      <Typography
        className={classes.headerChapterText}
        variant="subtitle1"
        component="h2"
        gutterBottom
      >
        {chapterName.toUpperCase()}
      </Typography>
      <Typography className={classes.headerMapText} variant="h4" component="h2">
        {mapName.toUpperCase()}
      </Typography>
      <img style={{ width: "100%" }} src={image} alt="level art"></img>
    </Grid>
  );
}

export default Header;
