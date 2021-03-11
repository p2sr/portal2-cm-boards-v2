import { BrowserRouter as Router, Switch, Route } from "react-router-dom"
import Home from "./Home/Home"
import "./body.css"

const Body = () => {
  return (
    <div className='body'>
      <Router>
        <Switch>
          {/* Insert the routes to other pages here as: 
              <Route path='/(page name) component={(component name)}*/}
          <Route exact path='/' component={Home} />
          <Route component={Error} />
        </Switch>
      </Router>
    </div>
  )
}

export default Body
