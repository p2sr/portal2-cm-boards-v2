import React, { useState } from "react";
import Home from "./Home";
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
function Router() {
  return (
    <ThemeProvider>
      <UserProvider>
        <SignupProvider>
          <div>
            <Router>
              <Nav />
              <Switch>
                {/* Insert the routes to other pages here as: 
                  <Route path='/(page name) component={(component name)}*/}
                <Route exact path='/' component={Home} />
                <Route component={Error} />
              </Switch>
              <Footer />
            </Router>
          </div>
        </SignupProvider>
      </UserProvider>
    </ThemeProvider>
  );
}

export default Router;
