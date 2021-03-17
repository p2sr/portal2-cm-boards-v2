import error404 from "./img/404.jpg"
import React from "react"
import { makeStyles, withStyles } from "@material-ui/core/styles"

/**
 * @name - Error
 * @desc - Error page. Displays the error img with a grey filler
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

// Styling for use of the error theme.
const useStyles = makeStyles(theme => ({
  root: {},
  error: {
    backgroundColor: "rgb(31,31,31)",
    height: "100vh"
  },
  errorImg: {
    height: "auto",
    width: "100%"
  }
}))

// disables the header and footer so only the error image and background is displayed.
const disableHeaderFooter = e => {
  e.preventDefault()
  document.getElementById("container-header").style.display = "none"

  document.getElementById("container-footer").style.display = "none"
}

const Error = () => {
  const classes = useStyles()
  return (
    <div onLoad={disableHeaderFooter} on className={classes.error}>
      <img src={error404} className={classes.errorImg} />
    </div>
  )
}

export default Error
