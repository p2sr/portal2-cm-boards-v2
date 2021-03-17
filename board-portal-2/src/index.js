import React from "react"
import ReactDOM from "react-dom"
import App from "./App"
import { ThemeProvider } from "@material-ui/core/styles"
import CssBaseline from "@material-ui/core/CssBaseline"
import theme from "./Theme"

/**
 * @name -
 * @desc -
 * @author -
 * @date -
 * @version -
 * @param -
 * @return -
 */
ReactDOM.render(
  <React.StrictMode>
    <CssBaseline />
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
  document.getElementById("root")
)
