import React from 'react'
import YoutubeEmbed from '../scenes/global/YoutubeEmbed'
import { Box } from '@mui/material'

const ScoreEntryDropdown = props => {   
    var submission = props.submission
  return (
    <Box display="flex">
        <YoutubeEmbed embedId={submission.youtube_id}/>
    </Box>
  )
}

export default ScoreEntryDropdown