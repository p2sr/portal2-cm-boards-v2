import { useStyles, CustomButton } from "./style.js"
import { Menu, MenuItem } from "@material-ui/core"
import sgIcon from "./img/Singleplayer.png"
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
      <CustomButton
        aria-controls='menu'
        aria-haspopup='true'
        onClick={handleClick}>
        <img src={aggregatedIcon} className={classes.icon} />
        Single Player
      </CustomButton>
      <Menu
        id='menu'
        getContentAnchorEl={null}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "center"
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "center"
        }}
        anchorEl={anchorEl}
        keepMounted
        open={Boolean(anchorEl)}
        onClose={handleClose}>
        <MenuItem onClick={handleClose} component={Link} to='/error'>
          PH 1
        </MenuItem>
        <MenuItem onClose={handleClose} component={Link} to='/error'>
          PH 2
        </MenuItem>
        <MenuItem onClose={handleClose} component={Link} to={error}>
          PH 3
        </MenuItem>
      </Menu>
    </div>
  )
}

export default DropdownSP
