import React, { useState, useEffect } from "react";
import { useStyles } from "./style";
import Grid from "@material-ui/core/Grid";
import ChamberCard from "./ChamberCard";
import courses from "./Courses";
import Typography from "@material-ui/core/Typography";
import imagePaths from "../SinglePlayer/ChamberImages";

const ENDPOINT = "http://localhost:8080/api/v1/coop";

/**
 * @name - App
 * @desc - React component that displays all cooperative maps with their top scores.
 * @author - John Fiedler
 * @date - 4/6/21
 * @version - 1.0
 * @param -
 * @return -
 */

function Cooperative() {
  const classes = useStyles();
  const [levelData, setLevelData] = useState([]);

  //fetching level data on first component load
  useEffect(() => {
    const fetchData = async () => {
      try {
        let response = await fetch(ENDPOINT);
        let data = await response.json();
        setLevelData(data);
      } catch (error) {
        console.log(error);
      }
    };

    fetchData();
  }, []);

  return (
    <div>
      {/*---------------Course 01------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 01
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          TEAM BUILDING
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(0, 6).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>

      {/*---------------Course 02------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 02
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          MASS AND VELOCITY
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(6, 14).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>

      {/*---------------Course 03------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 03
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          HARD LIGHT
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(14, 22).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>

      {/*---------------Course 04------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 04
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          EXCURSION FUNNELS
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(22, 31).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>

      {/*---------------Course 05------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 05
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          MOBILITY GELS
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(31, 39).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>

      {/*---------------Course 06------------------*/}
      <div className={classes.chapter_title_container}>
        <Typography
          variant="h6"
          component="h1"
          className={classes.chapter_number}
        >
          Course 06
        </Typography>
        <Typography
          variant="h4"
          component="h1"
          className={classes.chapter_name}
        >
          ART THERAPY
        </Typography>
      </div>
      <Grid
        container
        spacing={2}
        direction="row"
        className={classes.chapter_container}
      >
        {levelData.slice(39).map((level) => {
          return (
            <Grid key={level.map_name} item>
              <ChamberCard
                level_id={level.scores[0].map_id}
                scores={level.scores}
                image={imagePaths[level.scores[0].map_id]}
                title={level.map_name}
              />
            </Grid>
          );
        })}
      </Grid>
    </div>
  );
}

export default Cooperative;
