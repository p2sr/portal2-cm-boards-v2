import * as React from 'react';
import Topbar from "../global/Topbar";
import { Box, useTheme, Typography, MenuItem, FormControl, Select } from "@mui/material";
import { tokens } from "../../theme";
import { leaderboardCategories } from "../global/NavItems";
import { useEffect, useState } from "react";
import LeaderboardEntries from '../../components/LeaderboardEntries';
import { aggTimeLeaderboard, aggPointsLeaderboard } from './DummyDataLB';

const OVERALL_POINTS_ENDPOINT = "http://localhost:8080/api/v1/points/overall"

const Overall = () => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);

    const [category, setCategory] = React.useState(0);
    
    const categories = leaderboardCategories.map (category => {
        return <MenuItem value={category.id}>
            {category.title}
        </MenuItem>
    })

    const [overallPointsData, setoverallPointsData] = useState([]);
    const [loading, setLoading] = useState(true)
    
    //fetching changelog data on first component load
    useEffect(() => {
        const fetchData = async () => {
            try {
                const overallPointsResponse = await Promise.all([
                    fetch(OVERALL_POINTS_ENDPOINT).then(response => {
                        if (!response.ok) {
                            throw new Error('Changelog response not OK');
                        }
                        return response.json();
                    })
                ]);
    
                setoverallPointsData(overallPointsResponse);
                setLoading(false)
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };
        fetchData();
    }, []);

    const handleChange = (event) => {
        setCategory(event.target.value);
      };

    return <div id="main" flexDirection="column" justifyContent="center" style={{"--bgcolor": theme.palette.background.default}}>
        <Topbar/>
        <Typography
                variant="h3"
                color={colors.gray[100]}
                fontWeight="regular"
                align="center"
                >
                {"AGGREGATED POINTS AND TIMES"}
        </Typography>
        <Typography
                variant="h1"
                color={colors.gray[100]}
                fontWeight="bold"
                align="center"
                >
                {"OVERALL"}
        </Typography>
        <Box display="flex" flexDirection="column" justifyContent="center" alignItems="center">
            <Box sx={{ width: 200 }} marginTop={2} marginBottom={3}>
                <FormControl fullWidth>
                    <Select
                    value={category}
                    onChange={handleChange}
                    sx={{border: '3px solid rgba(63, 65, 69, 255)', borderRadius: 3, backgroundColor: "#151617"}}
                    displayEmpty
                    >
                    {categories}
                    </Select>
                </FormControl>
            </Box>
        </Box>
        <Box display="flex" flexDirection="row" justifyContent="center" alignItems="flex-start">

            {loading ? null :
                <LeaderboardEntries
                data={overallPointsData[0].points}
                type={1}
                />
            }
            {loading ? null :
                <LeaderboardEntries
                data={overallPointsData[0].points}
                type={2}
                />
            }
        </Box>
    </div>
}

export default Overall;