import { makeStyles, withStyles } from "@material-ui/core/styles"
import { Button } from "@material-ui/core"

export const CustomButton = withStyles({
  root: {
    fontFamily: [
      "-apple-system",
      "BlinkMacSystemFont",
      '"Segoe UI"',
      "Roboto",
      '"Helvetica Neue"',
      "Arial",
      "sans-serif",
      '"Apple Color Emoji"',
      '"Segoe UI Emoji"',
      '"Segoe UI Symbol"'
    ].join(",")
  }
})(Button)

export const useStyles = makeStyles(theme => ({
  root: {},
  appBar: {},
  toolbar: {
    minHeight: "8.033em",
    alignItems: "flex-start",
    paddingTop: theme.spacing(1),
    paddingBottom: theme.spacing(2)
  },
  title: {
    flexGrow: 1,
    alignSelf: "auto",
    paddingBottom: "1em"
  },
  headerLinks: {
    // width: "100%"
  },
  steam: {
    justifyContent: "flex-end",
    alignSelf: "center",
    "& div": {
      fontSize: "1.25rem",
      padding: ".5em"
    }
  },
  icon: {
    height: "1.56em",
    paddingLeft: "0.5em",
    paddingRight: "0.5em"
  },
  iconMenu: {
    height: "1.56em",
    width: "1.56em"
  },
  dropdown: {}
}))
