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
  toolbar: {
    minHeight: 128,
    alignItems: "flex-start",
    paddingTop: theme.spacing(1),
    paddingBottom: theme.spacing(2)
  },
  title: {
    flexGrow: 1,
    alignSelf: "auto",
    paddingBottom: "1em"
  },
  steam: {
    justifyContent: "space-around",
    alignSelf: "center",
    width: "10%"
  },
  icon: {
    height: "25px",
    paddingLeft: "0.5em",
    paddingRight: "0.5em"
  }
}))
