import { makeStyles, withStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  root: {},
  bodyPage: {
    margin: "4%",
    paddingTop: "0.5em",
    paddingLeft: "2em",
    paddingBottom: "56px"
    // height: "100vh"
  },
  cellDate: {
    width: "10.625em"
  },
  cellPlayer: {
    maxWidth: "15em",
    minWidth: "15em"
  },
  cellMap: {
    width: "7.5em"
  },
  cellChapter: {
    width: "7.5em"
  },
  cellPS: {
    width: "7.5em"
  },
  cellNS: {
    width: "7.5em"
  },
  cellImprovement: {
    width: "7.5em"
  },
  textField: { margin: ".5em" },
  buttonMenu: {},
  menu: {},
  menuItem: {},
  checkbox: {},
  apply: {}
}))
