import React, { useState, useContext } from "react"
import { BrowserRouter as Router, Switch, Route } from "react-router-dom"
import { ThemeProvider, createMuiTheme } from "@material-ui/core/styles"
import { useMediaQuery, Box } from "@material-ui/core"
// import { theme } from "./Theme"
import Footer from "./components/Footer/Footer"
import Header from "./components/Header/Header"
import Changelog from "./components/Body/ChangeLog/ChangeLog"
import About from "./components/Body/About/About"
import WallOfShame from "./components/Body/Wall_of_Shame/WallOfShame"
import Error from "./components/Error"
import Donators from "./components/Body/Donators/Donators"
import SinglePlayer from "./components/Body/SinglePlayer/SinglePlayer"
import Cooperative from "./components/Body/Cooperative/Cooperative"
import { useStyles } from "./style.js"
import AggregatedSelector from "./components/Body/Aggregated_Selector/AggregatedSelector"
import AggregatedOverall from "./components/Body/Aggregated_Overall/AggregatedOverall"
import MapPage from "./components/Body/MapPage/MapPage"

/**
 * @name - App
 * @desc - React Component that holds the theme, Header, Body(Routes), and Footer components
 *
 *        (Currently defaults to light mode as in dev mode, useMediaQuery is ran twice with
 *          the first value being false, should work in production.)
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

export const ThemeContext = React.createContext({})

function App() {
  const classes = useStyles()
  const [themeStatus, setThemeStatus] = React.useState(
    !localStorage.getItem("localTheme") || ""
    // !useMediaQuery("(prefers-color-scheme: dark)")
  )

  // console.log("current state =", themeStatus)

  const lightTheme = createMuiTheme({
    palette: {
      type: "light",
      primary: {
        main: "#82b1ff"
      },
      secondary: {
        main: "#ffab40"
      }
    }
  })
  const darkTheme = createMuiTheme({
    palette: {
      type: "dark",
      primary: {
        main: "#303030"
      },
      secondary: {
        main: "#303030"
      }
    }
  })

  const theme = themeStatus ? lightTheme : darkTheme

  const handleChange = event => {
    setThemeStatus(event.target.checked)
    localStorage.setItem("localTheme", themeStatus)
  }

  return (
    <Box bgcolor={themeStatus ? "rgb(154, 166, 187)" : "rgb(41, 49, 62)"}>
      <ThemeProvider theme={theme}>
        <ThemeContext.Provider value={{ theme, themeStatus }}>
          <Router>
            <Header handleChange={handleChange} themeStatus={themeStatus} />
            <Switch>
              {/* Insert the routes to other pages here as:
              <Route path='/(page name) component={(component name)}*/}
              <Route exact path='/' component={Changelog} />
              <Route path='/about' component={About} />
              <Route path='/agg-selector' component={AggregatedSelector} />
              <Route path='/agg-overall' component={AggregatedOverall} />
              <Route path='/donators' component={Donators} />
              <Route path='/wall_of_shame' component={WallOfShame} />
              <Route path='/sp' exact component={SinglePlayer} />
              <Route path='/coop' exact component={Cooperative} />
              <Route path='/sp/:map_id' component={MapPage} />
              <Route path='/coop/:map_id' component={MapPage} />
              <Route component={Error} />
            </Switch>
            <Footer />
          </Router>
        </ThemeContext.Provider>
      </ThemeProvider>
    </Box>
  )
}

export default App
