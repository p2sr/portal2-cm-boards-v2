import React from 'react';
import { useStyles } from './style';
import Grid from '@material-ui/core/Grid';
import ChamberCard from './ChamberCard';
import courses from './Courses';
import Typography from '@material-ui/core/Typography';

/**
 * @name - App
 * @desc - React component that displays all cooperative maps with their top scores.
 * @author - John Fiedler
 * @date - 4/6/21
 * @version - 1.0
 * @param -
 * @return -
 */

function Cooperative(){
    const classes = useStyles();

    return(
        <div>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 01</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Team Building</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}
            >
                {courses.course_1.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 02</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Mass and Velocity</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {courses.course_2.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id} image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 03</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Hard Light</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {courses.course_3.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 04</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Excursion Funnels</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {courses.course_4.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 05</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Mobility Gels</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {courses.course_5.map(level => {
                    return(
                        <Grid key={level.id} item>
                            <ChamberCard level_id={level.id}  image={level.image} title={level.title}/>
                        </Grid>
                    )
                })}
            </Grid>
            <div className={classes.chapter_title_container}>
                <Typography variant="h6" component="h1" className={classes.chapter_number}>Course 06</Typography>
                <Typography variant="h4" component="h1" className={classes.chapter_name}>Art Therapy</Typography>
            </div>
            <Grid 
            container 
            spacing={2}
            direction="row"
            className={classes.chapter_container}    
            >
                {courses.course_6.map(level => {
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

export default Cooperative;