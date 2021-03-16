import React, { useState } from "react"
import Footer from "./components/Footer/Footer"
import Header from "./components/Header/Header"
import Home from "./components/Body/Home/Home"
import About from "./components/Body/About/About"
import { BrowserRouter as Router, Switch, Route } from "react-router-dom"
import Error from "./components/Error"

/**
 * @name -
 * @desc -
 * @author -
 * @date -
 * @version -
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
          <Route component={Error} />
        </Switch>
        <Footer />
      </Router>
    </div>
  )
}

export default App
