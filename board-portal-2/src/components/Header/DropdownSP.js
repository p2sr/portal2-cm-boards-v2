import { useStyles, CustomButton } from "./style.js"
import { Menu, MenuItem } from "@material-ui/core"
import sgIcon from "./img/Singleplayer.png"
import spIcon from "./img/Singleplayer.png"
import React from "react"
import { Link } from "react-router-dom"

const DropdownSP = () => {
  const classes = useStyles()
  const [anchorEl, setAnchorEl] = React.useState(null)

  const handleClick = event => {
    setAnchorEl(event.currentTarget)
  }

  const handleClose = () => {
    setAnchorEl(null)
  }

  return (
    <div id='container' className={classes.dropdown}>
      <div aria-controls='menu' aria-haspopup='true' onClick={handleClick}>
        <img src={spIcon} className={classes.iconMenu} />
        Single Player
      </div>
      <Menu
        id='menu'
        getContentAnchorEl={null}
        anchorOrigin={{
          vertical: "top",
          horizontal: "right"
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right + 250px"
        }}
        anchorEl={anchorEl}
        keepMounted
        open={Boolean(anchorEl)}
        onClose={handleClose}>
        <MenuItem onClose={handleClose} component={Link} to='/'>
          PH 1
        </MenuItem>
        <MenuItem onClose={handleClose} component={Link} to='/'>
          PH 2
        </MenuItem>
        <MenuItem onClose={handleClose} component={Link} to='/'>
          PH 3
        </MenuItem>
      </Menu>
    </div>
  )
}

export default DropdownSP
