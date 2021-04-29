import React, { useState, useEffect } from "react";
import { useStyles } from "./style";
import Grid from "@material-ui/core/Grid";
import SingleChapters from "./Chapters";
import Header from "./Header";
import Scores from "./Scores";

const ENDPOINT = "http://localhost:8080/api/maps";

function MapPage({ match }) {
  const [mapData, setMapData] = useState([]);
  const classes = useStyles();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(`${ENDPOINT + match.url}`);
        const data = await response.json();

        setMapData(data);
      } catch (error) {
        console.log(error);
      }
    };

    fetchData();
  }, []);

  return (
    <Grid container justify="center">
      <Grid
        className={classes.pageContainer}
        container
        direction="column"
        alignItems="center"
        xs={12}
        md={6}
      >
        <Grid item xs={12}>
          <Header
            mapName={SingleChapters[match.params.map_id].title}
            chapterName={SingleChapters[match.params.map_id].chapter_name}
            image={SingleChapters[match.params.map_id].image}
          />
        </Grid>
        <Grid className={classes.scores} item xs={12}>
          <Scores
            mapData={mapData}
            mapType={match.url.indexOf("sp") > -1 ? "sp" : "coop"}
          />
        </Grid>
      </Grid>
    </Grid>
  );
}

export default MapPage;
