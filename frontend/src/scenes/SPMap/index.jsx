import * as React from 'react';
import Topbar from "../global/Topbar";
import YoutubeEmbed from '../global/YoutubeEmbed'
import { Box, useTheme, Typography, MenuItem } from "@mui/material";
import { tokens } from "../../theme";
import { leaderboardCategories } from "../global/NavItems";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import SPMapScoreEntries from '../../components/SPMapScoreEntries';
import mapInfo from '../../components/MapInfo';
import LocationOnIcon from '@mui/icons-material/LocationOn';
import chamberImages from '../global/ChamberImages'
import WRHistory from '../../components/WRHistory';

const SPMap = () => {

    function setBackground(levelId) {
        const backgroundImage = chamberImages[levelId];
        document.getElementById('main').style.backgroundImage = 
            `linear-gradient(to top, var(--bgcolor), rgba(255, 0, 0, 0)), url('${backgroundImage}')`;
    }
    
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);

    const { levelId } = useParams(); // Get the level ID from URL

    const [category, setCategory] = React.useState(0);
    
    const categories = leaderboardCategories.map (category => {
        return <MenuItem value={category.id}>
            {category.title}
        </MenuItem>
    })

    const [mapData, setMapData] = useState([]);
    const [loading, setLoading] = useState(true);
    
    //fetching changelog data on first component load
    useEffect(() => {
        const fetchData = async () => {
            try {
                const mapDataResponse = await Promise.all([
                    fetch(`http://localhost:8080/api/v1/map/sp/${levelId}`).then(response => {
                        if (!response.ok) {
                            throw new Error('Changelog response not OK');
                        }
                        return response.json();
                    })
                ]);
                setMapData(mapDataResponse);
                setLoading(false);
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };
        fetchData();
    }, []);

    var wrVid = "";

    if (!loading) {
        wrVid = mapData[0].find(point => point.map_data.youtube_id !== undefined);
    }
    
    const handleChange = (event) => {
        setCategory(event.target.value);
      };

    return <div id="main" flexDirection="column" justifyContent="center" style={{"--bgcolor": theme.palette.background.default}}>
        <Topbar/>

        {/* Map info */}
        <Box display="flex" justifyContent="center" padding="15px">
            <Box display="flex"
            justifyContent="center"
            flexDirection="column"
            width="60%"
            style={{borderRadius:"10px"}}
            >
                {/* Header */}
                <Box
                display="flex"
                padding="10px"
                flexGrow="1"
                backgroundColor={colors.primary[600]}
                style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                alignItems="center"
                >
                    <LocationOnIcon style={{fontSize:"200%"}}/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="regular"
                        sx={{m : "0 0 0 10px" }}
                        >
                        MAP INFO
                    </Typography>
                </Box>

                {/* Map info */}
                <Box
                display="flex"
                flexDirection="row"
                flexGrow="1"
                backgroundColor={colors.primary[700]}
                alignContent="flex-end"
                >
                    <Box
                    display="flex"
                    flexDirection="column"
                    marginTop={4}
                    >
                        <Typography
                            variant="h1"
                            color={colors.gray[100]}
                            fontWeight="bold"
                            sx={{m : "0 0 0 20px" }}
                            >
                            {mapInfo[levelId].title.toUpperCase()}
                        </Typography>
                        <Typography
                            variant="h2"
                            color={colors.gray[100]}
                            fontWeight="regular"
                            sx={{m : "0 0 0 20px" }}
                            >
                            {mapInfo[levelId].chapter_name.toUpperCase()}
                        </Typography>
                        {/* WRHistory */}
                        <div
                            display="flex"
                            flexGrow="1"
                            backgroundColor={colors.primary[700]}
                            style={{
                                backgroundColor:colors.primary[700],
                                width:"100%",
                                backgroundClip:"padding-box",
                                height: "250px",
                            }}
                            alignItems="center"
                            justifyContent="center"
                            >
                            <WRHistory levelId={levelId}/>
                        </div>
                    </Box>
                    <Box>
                        {loading ? null :
                            <YoutubeEmbed embedId={wrVid.map_data.youtube_id}/>
                        }
                    </Box>
                </Box>

                {/* Scores */}
                <div
                    display="flex"
                    padding="20px"
                    flexGrow="1"
                    backgroundColor={colors.primary[600]}
                    style={{
                        borderBottomLeftRadius:"10px",
                        backgroundColor:colors.primary[600],
                        borderBottomRightRadius:"10px",
                        width:"100%",
                        padding:"20px",
                        backgroundClip:"padding-box"
                    }}
                    alignItems="center"
                    justifyContent="center"
                    >
                    {loading ? null :
                        <SPMapScoreEntries mapData={mapData}/>
                    }
                </div>
            </Box>
        </Box>
    </div>
}

export default SPMap;