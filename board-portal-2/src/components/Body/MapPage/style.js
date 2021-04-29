import { FormHelperText } from "@material-ui/core";
import { grey } from "@material-ui/core/colors";
import { makeStyles } from "@material-ui/core/styles";

export const useStyles = makeStyles((theme) => ({
  root: {},
  pageContainer: {
    margin: "5vh 0 5vh 0",
    borderTop: `4px solid ${theme.palette.secondary.light}`,
  },
  headerMapText: {
    position: "absolute",
    marginTop: "6vh",
    color: theme.palette.common.white,
  },
  headerChapterText: {
    position: "absolute",
    color: theme.palette.common.white,
  },
  scores: {
    width: "100%",
    backgroundColor: theme.palette.background.paper,
  },
  scoreItem: {
    borderBottom: `1px solid ${theme.palette.primary.dark}`,
    color: theme.palette.text.primary,
    padding: "1vh 0 1vh 0",
  },
  scoreImage: {
    borderRadius: "100px",
    width: "35px",
    height: "35px",
  },
  scoreRank: {
    margin: "0 2vw 0 2vw",
  },
  scoreName: {
    marginLeft: "1vw",
  },
  scoreText: {
    marginRight: "1vw",
  },
}));
