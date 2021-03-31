import { makeStyles, withStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  root: {
    margin: "4%",
    paddingTop: "0.5em",
    paddingLeft: "2em",
    paddingBottom: "1em"
  },
  cards: {
    width: "100%",
    padding: "1.5em"
  },
  image: {
    position: "relative",
    height: 200,
    width: "100%",
    "&:hover, &$focusVisible": {
      zIndex: 1,
      "& $imageBackdrop": {
        opacity: 0.15
      },
      "& $imageMarked": {
        opacity: 0
      }
    }
  },
  focusVisible: {},
  imageButton: {
    position: "relative",
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    color: theme.palette.common.white
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
  imageBackdrop: {
    position: "absolute",
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundColor: theme.palette.common.black,
    opacity: 0.4,
    transition: theme.transitions.create("opacity")
  },
  imageTitle: {
    position: "relative",
    padding: `${theme.spacing(2)}px ${theme.spacing(4)}px ${
      theme.spacing(1) + 6
    }px`,
    fontSize: "2.75em",
    textTransform: "uppercase"
  }
}))
