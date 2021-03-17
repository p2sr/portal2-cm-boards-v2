import React, { useState } from "react"
import Footer from "./components/Footer/Footer"
import Header from "./components/Header/Header"
import Home from "./components/Body/Home/Home"
import About from "./components/Body/About/About"
import WallOfShame from "./components/Body/Wall_of_Shame/WallOfShame"
import { BrowserRouter as Router, Switch, Route } from "react-router-dom"
import Error from "./components/Error"
import Donators from "./components/Body/Donators/Donators"

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
  return (
    <div style={{ backgroundColor: "rgb(154, 166, 187)" }}>
      <Router>
        <Header />
        <Switch>
          {/* Insert the routes to other pages here as: 
              <Route path='/(page name) component={(component name)}*/}
          <Route exact path='/' component={Home} />
          <Route path='/about' component={About} />
          <Route path='/donators' component={Donators} />
          <Route path='/wall_of_shame' component={WallOfShame} />
          <Route component={Error} />
        </Switch>
        <Footer />
      </Router>
    </div>
  )
}

export default App
