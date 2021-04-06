import { makeStyles } from "@material-ui/core/styles"

export const useStyles = makeStyles(theme => ({
  root: {},
  aboutBody: {
    background: theme.palette.background.default,
    margin: "5%",
    paddingTop: "0.5em",
    paddingLeft: "2em",
    paddingBottom: "56px"
  },
  aboutHeader: {
    fontFamily: "arial",
    fontSize: " 2.5rem",
    color: theme.palette.text.primary,
    textTransform: "uppercase"
  },

  aboutSubHeader: {
    fontFamily: "arial",
    fontSize: "1.375rem",
    color: theme.palette.text.secondary,
    textTransform: "uppercase"
  },
  aboutText: {
    fontFamily: "arial",
    textTransform: "none",
    marginBottom: "30px",
    fontSize: "1rem",
    color: "#444",
    width: "calc(100% - 45px)",
    paddingLeft: "0",
    marginLeft: " 0",
    color: theme.palette.text.primary
  },

  fraction: {
    textTransform: "none",
    color: "black",
    fontSize: "0.9375rem",
    color: "inherit",
    margin: "0.9375em",
    textAlign: "center",
    verticalAlign: "middle",
    marginTop: "0.5em",
    marginBottom: "0.5em",
    lineHeight: "1.6em"
  },

  upperLine: {
    color: "black",
    fontSize: "0.9375em",
    color: "inherit",
    margin: "0.9375em",
    borderTop: "solid 1px #555555"
  },
  aboutTeamList: {
    color: theme.palette.text.primary
  }
}))
