import React, { useState } from "react"
import Footer from "./components/Footer/Footer"
import Header from "./components/Header/Header"
import Home from "./components/Body/Home/Home"
import About from "./components/Body/About/About"
import WallOfShame from "./components/Body/Wall_of_Shame/WallOfShame"
import { BrowserRouter as Router, Switch, Route } from "react-router-dom"
import Error from "./components/Error"
import Donators from "./components/Body/Donators/Donators"
import SinglePlayer from "./components/Body/SinglePlayer/SinglePlayer"
import { ThemeProvider, createMuiTheme } from "@material-ui/core/styles"
// import { theme } from "./Theme"

/**
 * @name - App
 * @desc - React Component that holds the Header, Body(Routes), and Footer components
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */
function App() {
  const [state, setState] = React.useState(true)

  console.log("current state =", state)

  const handleChange = event => {
    setState(event.target.checked)
  }
  const theme = createMuiTheme({
    palette: {
      type: state ? "light" : "dark",
      primary: {
        // light: "#9bc0ff",
        main: "#82b1ff",
        // dark: "#344666",
        contrastText: "rgb(101, 101, 101);"
      },
      secondary: {
        // light: "#ffbb66",
        main: "#ffab40",
        // dark: "#b2772c",
        contrastText: "rgb(246, 246, 246)"
      }
    }
  })

  return (
    <div style={{ backgroundColor: "rgb(154, 166, 187)" }}>
      <ThemeProvider theme={theme}>
        <Router>
          <Header handleChange={handleChange} state={state} />
          <Switch>
            {/* Insert the routes to other pages here as: 
              <Route path='/(page name) component={(component name)}*/}
            <Route exact path='/' component={Home} />
            <Route path='/about' component={About} />
            <Route path='/donators' component={Donators} />
            <Route path='/wall_of_shame' component={WallOfShame} />
            <Route path='/sp' component={SinglePlayer} />
            <Route component={Error} />
          </Switch>
          <Footer />
        </Router>
      </ThemeProvider>
    </div>
  )
}

export default App
