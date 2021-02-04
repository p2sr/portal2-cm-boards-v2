import React, { useState } from "react";
import Home from "./components/Home";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

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
    <div>
      <Router>
        <Switch>
          {/* Insert the routes to other pages here as: 
                  <Route path='/(page name) component={(component name)}*/}
          <Route exact path='/' component={Home} />
          <Route component={Error} />
        </Switch>
      </Router>
    </div>
  );
}

export default App;
