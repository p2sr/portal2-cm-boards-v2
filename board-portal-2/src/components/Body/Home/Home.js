import { Paper } from "@material-ui/core"
import React, { useState, useContext } from "react"
import { HashLink as Link } from "react-router-hash-link"
import { useStyles } from "./style.js"

/**
 * @name - Home
 * @desc - Home page for the website.
 *          TODO:
 *                - Add graph of upload activity
 *                - Display list of upload activity
 *                - Display options for filter
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */
function Home() {
  const classes = useStyles()
  return (
    <Paper id='Home' className={classes.bodyPage}>
      <h1>Welcome to the home page for Portal 2 Leaderboards </h1>
    </Paper>
  )
}

export default Home
