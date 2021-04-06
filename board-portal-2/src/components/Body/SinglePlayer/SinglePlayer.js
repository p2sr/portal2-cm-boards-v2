import React from 'react';
import { useStyles } from './style';
import Grid from '@material-ui/core/Grid';
import ChamberCard from './ChamberCard';
import chapters from './Chapters';
import Typography from '@material-ui/core/Typography';

/**
 * @name - App
 * @desc - React Component that holds the theme, Header, Body(Routes), and Footer components
 *
 *        (Currently defaults to light mode as in dev mode, useMediaQuery is ran twice with
 *          the first value being false, should work in production.)
 * @author - John Fiedler
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

function SinglePlayer(){
    const classes = useStyles();

    return(
        <div>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 01</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Courtesy Call</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}
            >
                {chapters.chapter_1.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 02</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Cold Boot</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_2.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id} image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 03</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Return</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_3.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 04</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Surprise</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_4.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 05</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Escape</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_5.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 06</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Fall</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_6.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 07</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Reunion</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_7.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 08</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Itch</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_8.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Chapter 09</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>The Part Where...</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {chapters.chapter_9.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
        </div>
    );
}

export default SinglePlayer;