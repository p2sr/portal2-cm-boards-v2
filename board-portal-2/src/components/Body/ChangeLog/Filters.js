import {
  AppBar,
  Grid,
  Menu,
  MenuItem,
  TextField,
  Toolbar,
  Typography,
  Button,
  Checkbox,
  FormGroup,
  FormControlLabel
} from "@material-ui/core"
import React from "react"
import { useStyles } from "./style"

const Filters = props => {
  const classes = useStyles()
  const [anchorEl, setAnchorEl] = React.useState(null)
  const [anchorEl2, setAnchorEl2] = React.useState(null)
  const [nickname, setNickname] = React.useState(null)
  const [steam, setSteam] = React.useState(null)
  const [chapter, setChapter] = React.useState(null)
  const [chamber, setChamber] = React.useState(null)
  const [check, setCheck] = React.useState({
    singlePlayer: false,
    cooperative: false,
    worldRecord: false,
    demo: false,
    video: false,
    submission: false
  })

  const handleClose1 = e => {
    setAnchorEl(null)
    setChapter(e.target.text)
  }
  const handleClose2 = e => {
    setAnchorEl2(null)
    setChamber(e.target.text)
  }

  const handleNickname = e => {
    setNickname(e.target.value)
  }
  const handleSteam = e => {
    setSteam(e.target.value)
  }

  const handleChange = e => {
    setCheck({ ...check, [e.target.name]: e.target.checked })
  }

  const handleOnClick = () => {
    props.onChangeFilters([nickname, steam, chapter, chamber, check])
  }

  return (
    <AppBar
      position='static'
      style={{
        backgroundColor: props.themeStatus
          ? "rgb(154, 166, 187)"
          : "rgb(41, 49, 62)"
      }}>
      <Toolbar>
        <form style={{ width: "100%", margin: props.theme.spacing(1) }}>
          <TextField
            onChange={handleNickname}
            className={classes.textField}
            label='Nickname'></TextField>
          <TextField
            className={classes.textField}
            onChange={handleSteam}
            label='Steam Profile number'></TextField>
          <Button
            className={classes.buttonMenu}
            name='el1'
            aria-controls='simple-menu'
            aria-haspopup='true'
            onClick={e => setAnchorEl(e.currentTarget)}>
            Chapters
          </Button>
          <Menu
            className={classes.menu}
            id='simple-menu'
            name='el1'
            anchorEl={anchorEl}
            keepMounted
            open={Boolean(anchorEl)}
            onClose={() => setAnchorEl(null)}>
            <MenuItem className={classes.menuItem} onClick={handleClose1}>
              No Chapters
            </MenuItem>
            <MenuItem className={classes.menuItem} onClick={handleClose1}>
              Chapter 1
            </MenuItem>
            <MenuItem className={classes.menuItem} onClick={handleClose1}>
              Chapter 2
            </MenuItem>
          </Menu>
          <Button
            className={classes.buttonMenu}
            name='el2'
            aria-controls='simple-menu'
            aria-haspopup='true'
            onClick={e => setAnchorEl2(e.currentTarget)}>
            Chambers
          </Button>
          <Menu
            className={classes.menu}
            id='simple-menu'
            name='el2'
            anchorEl={anchorEl2}
            keepMounted
            open={Boolean(anchorEl2)}
            onClose={() => setAnchorEl2(null)}>
            <MenuItem
              value='No Chambers'
              className={classes.menuItem}
              onClick={handleClose2}>
              No Chambers
            </MenuItem>
            <MenuItem className={classes.menuItem} onClick={handleClose2}>
              Chamber 1
            </MenuItem>
            <MenuItem className={classes.menuItem} onClick={handleClose2}>
              Chamber 2
            </MenuItem>
          </Menu>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.singlePlayer}
                onChange={handleChange}
                name='singlePlayer'
              />
            }
            label='Single Player'></FormControlLabel>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.cooperative}
                onChange={handleChange}
                name='cooperative'
              />
            }
            label='Cooperative'></FormControlLabel>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.worldRecord}
                onChange={handleChange}
                name='worldRecord'
              />
            }
            label='World Record'></FormControlLabel>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.demo}
                onChange={handleChange}
                name='demo'
              />
            }
            label='Demo'></FormControlLabel>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.video}
                onChange={handleChange}
                name='video'
              />
            }
            label='Video'></FormControlLabel>
          <FormControlLabel
            className={classes.checkbox}
            labelPlacement='top'
            control={
              <Checkbox
                color='default'
                checked={check.submission}
                onChange={handleChange}
                name='submission'
              />
            }
            label='Submission'></FormControlLabel>
        </form>
        <Button
          variant='outlined'
          className={classes.apply}
          onClick={handleOnClick}>
          Apply
        </Button>
      </Toolbar>
    </AppBar>
  )
}

export default Filters
