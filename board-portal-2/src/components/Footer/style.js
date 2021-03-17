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
    minHeight: 75,
    paddingTop: theme.spacing(1),
    paddingBottom: theme.spacing(2)
  },
  icon: {
    height: "1.568em",
    paddingLeft: "0.5em",
    paddingRight: "0.5em"
  }
}))
