import React, { useState, useEffect } from 'react';
import { useStyles } from './style';
import Grid from '@material-ui/core/Grid';
import ChamberCard from './ChamberCard';
import Typography from '@material-ui/core/Typography';
import imagePaths from './ChamberImages';

const ENDPOINT = 'http://localhost:8080/api/sp';

/**
 * @name - App
 * @desc - 
 * @author - John Fiedler
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

function SinglePlayer(){
    const classes = useStyles();
    const [levelData, setLevelData] = useState([]);

    //fetching level data on first component load
    useEffect(() => {
        const fetchData = async () => {
            let response = await fetch(ENDPOINT);

            return response.json();
        }

        fetchData().then(data => setLevelData(data))
        .catch(error => console.log(error));
    }, []);

    return(
        <div>
            {/*---------------Chapter 01------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 01</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE COURTESY CALL</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(0, 9).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 02------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 02</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE COLD BOOT</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(9, 17).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 03------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 03</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE RETURN</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(17, 26).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 04------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 04</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE SURPRISE</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(26, 31).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 05------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 05</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE ESCAPE</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(31, 35).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 06------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 06</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE FALL</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(35, 41).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 07------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 07</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE REUNION</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(41, 45).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 08------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 08</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE ITCH</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(45, 56).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>

            {/*---------------Chapter 09------------------*/}
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 09</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>THE PART WHERE...</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {levelData.slice(56).map(level => {
                    return(
                        <Grid key={level.map_name} item>
                            <ChamberCard level_id={level.scores[0].map_id} scores={level.scores} image={imagePaths[level.scores[0].map_id]} title={level.map_name}/>
                        </Grid>
                    )
                })}
            </Grid>
        </div>
    );
}

export default SinglePlayer;