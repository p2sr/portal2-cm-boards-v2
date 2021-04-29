import {
  Avatar,
  Divider,
  IconButton,
  List,
  ListItem,
  ListItemAvatar,
  ListItemSecondaryAction,
  ListItemText
} from "@material-ui/core"
import { YouTube, GetApp, ChatBubble } from "@material-ui/icons"
import React, { isValidElement } from "react"
import { useStyles } from "./style"
import chapters from "./Chapters"

const ScoreLists = props => {
  const classes = useStyles()
  var index = 0

  console.log(props.changelogData)
  console.log(props.filters)

  return (
    <List dense={true} style={{ paddingRight: "2em", width: "100%" }}>
      <ListItem>
        <ListItemText primary='Date' style={{ width: "7.67em" }} />
        <ListItemAvatar>
          <Avatar></Avatar>
        </ListItemAvatar>
        <ListItemText primary='Player' className={classes.cellPlayer} />
        <ListItemText primary='Map' />
        <ListItemText primary='Chapter' />
        <ListItemText primary='Previous Score' />
        <ListItemText primary='New Score' />
        <ListItemText primary='Improvement' />
        <ListItemSecondaryAction></ListItemSecondaryAction>
      </ListItem>

      {props.changelogData.map(submission => {
        var mapID = JSON.parse(submission.map_id)
        index++
        return (
          <ListItem
            style={{
              // hella not optimized but it works to alternate the color
              backgroundColor:
                index % 2 == 0
                  ? props.theme.palette.background.paper
                  : props.themeStatus
                  ? "rgb(154, 166, 187)"
                  : "rgb(41, 49, 62)"
            }}>
            <ListItemText
              primary={submission.time_gained}
              className={classes.cellDate}
            />
            <ListItemAvatar>
              <Avatar src={submission.avatar} />
            </ListItemAvatar>
            <ListItemText
              className={classes.cellPlayer}
              primary={
                submission.boardname
                  ? submission.boardname
                  : submission.steamname
              }
            />
            <ListItemText
              className={classes.cellMap}
              {...console.log(index)}
              primary={mapID != 47848 ? chapters[mapID].title : "DNE"}
            />
            <ListItemText
              className={classes.cellChapter}
              primary={submission.name}
            />
            <ListItemText
              className={classes.cellPS}
              primary={submission.pre_rank ? submission.pre_rank : 0}
            />
            <ListItemText
              className={classes.cellNS}
              primary={submission.post_rank}
            />
            <ListItemText
              className={classes.cellImprovement}
              primary={submission.post_rank - submission.pre_rank}
            />
            <ListItemSecondaryAction>
              {submission.note && (
                <IconButton edge='end'>
                  <ChatBubble />
                </IconButton>
              )}
              {submission.has_demo != 0 && (
                <IconButton edge='end'>
                  <GetApp />
                </IconButton>
              )}
              {submission.youtube_id && (
                <IconButton edge='end'>
                  <YouTube />
                </IconButton>
              )}
            </ListItemSecondaryAction>
          </ListItem>
        )
      })}
    </List>
  )
}

export default ScoreLists
