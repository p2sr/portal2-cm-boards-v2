import Card from '@material-ui/core/Card';
import CardActionArea from '@material-ui/core/CardActionArea';
import CardActions from '@material-ui/core/CardActions';
import CardContent from '@material-ui/core/CardContent';
import CardMedia from '@material-ui/core/CardMedia';
import Typography from '@material-ui/core/Typography';
import Grid from '@material-ui/core/Grid';
import { useStyles } from './style';
import { useEffect, useState } from 'react';
import axios from 'axios';

function ChamberCard(props){
    const classes = useStyles();
    const [cardData, setCardData] = useState([]);

    //runs once when component mounts
    useEffect(() => {
        const getData = async () => {
            let { data } = await axios.get(`http://localhost:5000/api/maps/sp/${props.level_id}`);

            let temp = data.slice(0, 7);
  
            let newData = temp.map(el => {
                return {
                    score: el.score_data.score,
                    name: el.user_data.boardname ? el.user_data.boardname : el.user_data.steamname
                }
            });

            setCardData(newData);
        }

        getData();
    }, []);

    return(
        <Card className={classes.chamber_card}>
            <CardActionArea>
                <CardMedia
                    className={classes.chamber_img}
                    image={props.image}
                    title="Container Ride"
                />
            </CardActionArea>
            <div className={classes.level_title_helper}>
            </div>
            <div className={classes.level_title}>
                {props.title}
            </div>
            <CardContent className={classes.first_place}>
                <Grid 
                    container
                    direction="row"
                    justify="space-between"
                >
                    <Typography variant="body2">
                        {cardData.length > 0 ? cardData[0].name : "null"}
                    </Typography>
                    <Typography variant="body2">
                        {cardData.length > 0 ? cardData[0].score : "null"}
                    </Typography>
                </Grid>
            </CardContent>
            {cardData.map((score,i) => {
                if(i > 0){
                    return (
                        <CardContent key={i} className={classes.card_content}>
                            <Grid 
                                container
                                direction="row"
                                justify="space-between"
                            >
                                <Typography variant="caption">
                                    {score.name}
                                </Typography>
                                <Typography variant="caption">
                                    {score.score}
                                </Typography>
                            </Grid>
                        </CardContent>
                    )
                }
            })}
        </Card>
    );
}

export default ChamberCard;