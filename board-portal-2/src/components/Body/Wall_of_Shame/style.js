import { makeStyles, withStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  bodyWallOfShame: {
    background: theme.palette.background.default,
    margin: "5%",
    paddingTop: "0.5em",
    paddingLeft: "2em",
    paddingBottom: "56px"
  },
  titleWallOfShame: {
    textTransform: "uppercase",
    fontSize: "2.5rem",
    fontFamily: "Arial",
    fontWeight: "600"
  },
  subWallOfShame: {
    textTransform: "uppercase",
    fontFamily: "Arial",
    fontWeight: "600"
  },
  list: {
    width: "15%"
  },
  listItem: {
    width: "inherit",
    justifyContent: "space-around"
  }
}))
