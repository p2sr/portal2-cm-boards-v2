import { makeStyles, withStyles } from "@material-ui/core/styles"
import { IconButton } from "@material-ui/core"

export const CustomIconButton = withStyles({
  root: {
    color: "#fff",
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
})(IconButton)

export const useStyles = makeStyles(theme => ({
  root: {
    margin: "4%",
    // paddingLeft: "2em",
    paddingBottom: "56px"
  },
  image: {
    position: "relative",
    height: 200,
    width: "100%"
  },
  imageSrc: {
    position: "absolute",
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundSize: "cover",
    backgroundPosition: "center 100%"
  },
  imageTitle: {
    position: "relative",
    fontSize: "2em",
    textTransform: "uppercase",
    color: "#fff"
  },
  imageSubTitle: {
    position: "relative",
    fontSize: "1.75em",
    marginTop: "1em",
    textTransform: "uppercase",
    color: "#fff"
  },
  showingSelector: {
    position: "relative",
    color: "#fff"
  },
  lists: {
    padding: "1em"
  }
}))
