import { createMuiTheme, getContrastRatio } from "@material-ui/core/styles"

/**
 * @name -
 * @desc -
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

const theme = createMuiTheme({
  palette: {
    primary: {
      light: "#9bc0ff",
      main: "#82b1ff",
      dark: "#5b7bb2",
      contrastText: "#fff"
    },
    secondary: {
      light: "#ffbb66",
      main: "#ffab40",
      dark: "#b2772c",
      contrastText: "#ddd"
    }
  }
})
export default theme
