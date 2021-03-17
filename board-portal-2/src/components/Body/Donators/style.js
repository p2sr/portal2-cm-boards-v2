import { makeStyles, withStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  bodyDonators: {
    background: "rgb(246,246,246)",
    margin: "5%",
    paddingTop: "0.5em",
    paddingLeft: "2em",
    paddingBottom: "56px"
  },
  titleDonators: {
    textTransform: "uppercase",
    fontSize: "2.5rem",
    fontFamily: "Arial",
    fontWeight: "600"
  },
  listItem: {
    width: "inherit",
    justifyContent: "space-around"
  },
  img: {
    width: "30%",
    justifyContent: "flex-end"
  }
}))
