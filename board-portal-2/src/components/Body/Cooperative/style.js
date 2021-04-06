import { FormHelperText } from "@material-ui/core"
import { grey } from "@material-ui/core/colors"
import { makeStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  root: {},
  card_content: {
    paddingBottom: 2,
    paddingTop: 2,
    "&:last-child": {
      paddingBottom: 2
    },
    backgroundColor: theme.palette.background.paper
  },
  chapter_number: {
    color: "white",
    marginBottom: -2
  },
  chapter_name: {
    color: "white",
    marginTop: -2
  },
  chapter_container: {
    maxWidth: "100%",
    maxHeight: "100%",
    paddingLeft: "1.675em",
    paddingBottom: ".675em",
    marginBottom: "2%"
  },
  chapter_title_container: {
    display: "flex",
    flexDirection: "column",
    marginLeft: 26,
    marginTop: 20,
    marginBottom: 15
  },
  chamber_card: {
    maxWidth: 265,
    width: 265,
    position: "relative",
    borderTop: `4px solid ${theme.palette.secondary.light}`
  },
  chamber_img: {
    height: 149
  },
  level_title: {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    position: "absolute",
    top: 0,
    right: 0,
    height: 30,
    width: "50%",
    color: "black",
    backgroundColor: "#dbdbdb",
    padding: "0px 10px 0px 10px"
  },
  level_title_helper: {
    position: "absolute",
    top: 0,
    right: "50%",
    width: 0,
    height: 0,
    borderTop: "30px solid #dbdbdb",
    borderLeft: "30px solid transparent"
  },
  first_place: {
    backgroundColor: theme.palette.background.default,
    width: "100%"
  }
}))
