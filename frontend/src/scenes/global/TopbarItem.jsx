import React from 'react'
import { Link } from 'react-router-dom';
import { Box, Typography, useTheme} from "@mui/material";
import { useState } from "react";
import { tokens } from "../../theme";
import Dropdown from "./Dropdown";
import "./TopbarItem.css";

const TopbarItem = props => {
  const theme = useTheme(); 
  const colors = tokens(theme.palette.mode);

  const [toggle, setToggle] = useState(false);

  return (
    <Box className="topbar-item"
    component={Link}
    to={props.path}
    color="inherit"
    onMouseEnter={() => setToggle(true)}
    onMouseLeave={() => setToggle(false)}
    >
        {props.icon}
        <Typography
            variant="h4"
            color={colors.gray[100]}
            fontWeight="medium"
            sx={{m : "0 0 0 10px" }}
            >
            {props.title}
            <Dropdown dropdown={props.dropdown} toggle={toggle}/>
        </Typography>
    </Box>
  )
}

export default TopbarItem