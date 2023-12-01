import { React } from 'react'
import { Link } from 'react-router-dom'
import "./Dropdown.css";
import { Typography, useTheme} from "@mui/material";
import { tokens } from "../../theme";

const Dropdown = props => {

    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    if (props.dropdown !== null) {
    return (
        <>
            <div
            className={props.toggle ? "services-submenu" : "services-submenu clicked"}
            >
                {props.dropdown.map(item => {
                    return (
                        <li key={item.id}>
                            <Link to={item.path}
                            className={item.cName}
                            >
                                <Typography
                                    variant="h4"
                                    color={colors.gray[100]}
                                    fontWeight="regular" 
                                    sx={{m : "0 0 0 0px" }}
                                    >
                                    {item.title}
                                </Typography>
                            </Link>
                        </li>
                    )
                })}
            </div>
        </>
    )
    } else {
        return <></>
    }
}

export default Dropdown