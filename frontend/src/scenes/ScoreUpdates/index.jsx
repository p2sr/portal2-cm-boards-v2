import Topbar from "../global/Topbar";
import React from "react";
import { Box, useTheme, Typography } from "@mui/material";
import { tokens } from "../../theme";
import { useEffect, useState } from "react";
import './index.css';
import HistoryIcon from '@mui/icons-material/History';
import ScoreEntries from '../../components/ScoreEntries'
import ScoreGraph from '../../components/ScoreGraph'

const CHANGELOG_ENDPOINT = "http://localhost:8080/api/v1/changelog"
const GRAPH_ENDPOINT = "http://localhost:8080/api/v1/graph"


const ScoreUpdates = () => {
    const [changelogData, setChangelogData] = useState([]);
    const [graphData, setGraphData] = useState([]); 
    const [loading, setLoading] = useState(true)
    
    //fetching changelog data on first component load
    useEffect(() => {
        const fetchData = async () => {
            try {
                const [changelogResponse, graphResponse] = await Promise.all([
                    fetch(CHANGELOG_ENDPOINT).then(response => {
                        if (!response.ok) {
                            throw new Error('Changelog response not OK');
                        }
                        return response.json();
                    }),
                    fetch(GRAPH_ENDPOINT).then(response => {
                        if (!response.ok) {
                            throw new Error('Graph response not OK');
                        }
                        return response.json();
                    })
                ]);
    
                setChangelogData(changelogResponse);
                setGraphData(graphResponse);
                setLoading(false)
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };
        fetchData();
    }, []);
    

    const theme = useTheme();
    const colors = tokens(theme.palette.mode);
    return <div id="main" flexDirection="column" justifyContent="flex-start" style={{"--bgcolor": theme.palette.background.default}}>
        <Topbar />
        {/* Info boxes */}
        <Box display="flex" justifyContent="center" padding="15px">
            <Box display="flex" justifyContent="center" width="93%" gap="30px">
                {/* Following*/}
                <Box display="flex" justifyContent="center" flexGrow={0.3}>
                    <Box display="flex"
                    justifyContent="center"
                    flexDirection="column"
                    width="100%"
                    >
                        {/* Title: FOLLOWING */}
                        <Box
                        display="flex"
                        padding="10px"
                        backgroundColor={colors.primary[700]}
                        style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                        alignItems="center"
                        >
                            <HistoryIcon style={{fontSize:"200%"}}/>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                sx={{m : "0 0 0 10px" }}
                                >
                                FOLLOWING
                            </Typography>
                        </Box>

                        {/* Following Scores */}
                        <div
                        display="flex"
                        backgroundColor={colors.primary[600]}
                        style={{
                            borderBottomLeftRadius:"10px",
                            backgroundColor:colors.primary[600],
                            borderBottomRightRadius:"10px",
                            backgroundClip:"padding-box"
                        }}
                        alignItems="center"
                        justifyContent="center"
                        >
                            {loading ? null :
                                <ScoreGraph
                                graphData={graphData}
                                />
                            }
                        </div>
                    </Box>
                </Box>
                {/* Daily Activity */}
                <Box display="flex" justifyContent="center" flexGrow={0.7}>
                    <Box display="flex"
                    justifyContent="center"
                    flexDirection="column"
                    width="100%"
                    >
                        {/* Title: DAILY ACTIVITY */}
                        <Box
                        display="flex"
                        padding="10px"
                        backgroundColor={colors.primary[700]}
                        style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                        alignItems="center"
                        >
                            <HistoryIcon style={{fontSize:"200%"}}/>
                            <Typography
                                variant="h5"
                                color={colors.gray[100]}
                                fontWeight="regular"
                                sx={{m : "0 0 0 10px" }}
                                >
                                DAILY ACTIVITY
                            </Typography>
                        </Box>

                        {/* Graph */}
                        <div
                        display="flex"
                        backgroundColor={colors.primary[600]}
                        style={{
                            borderBottomLeftRadius:"10px",
                            backgroundColor:colors.primary[600],
                            borderBottomRightRadius:"10px",
                            backgroundClip:"padding-box"
                        }}
                        alignItems="center"
                        justifyContent="center"
                        >
                            {loading ? null :
                                <ScoreGraph
                                graphData={graphData}
                                />
                            }
                        </div>
                    </Box>
                </Box>
            </Box>
        </Box>

        {/* Score Updates */}
        <Box display="flex" justifyContent="center" padding="15px">
            <Box display="flex"
            justifyContent="center"
            flexDirection="column"
            width="93%"
            style={{borderRadius:"10px"}}
            >
                {/* Title: Score Updates */}
                <Box
                display="flex"
                padding="10px"
                flexGrow="1"
                backgroundColor={colors.primary[700]}
                style={{borderTopLeftRadius:"10px", borderTopRightRadius:"10px"}}
                alignItems="center"
                >
                    <HistoryIcon style={{fontSize:"200%"}}/>
                    <Typography
                        variant="h5"
                        color={colors.gray[100]}
                        fontWeight="regular"
                        sx={{m : "0 0 0 10px" }}
                        >
                        SCORE UPDATES
                    </Typography>
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
                    <ScoreEntries
                        changelogData={changelogData}
                    />
                </div>
            </Box>
        </Box>
    </div>
}

export default ScoreUpdates;