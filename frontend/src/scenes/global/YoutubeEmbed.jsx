import React from 'react'

const YoutubeEmbed = ({embedId}) => {
  return (
    <div style={{width:"100%", height:"100%"}}>
        <iframe
        width="100%"
        height="250px"
        src={`https://www.youtube.com/embed/${embedId}`}
        allowFullScreen
        frameBorder="none"
        title='YouTube embed'
        style={{border:'2px solid #3F4145', borderRadius:"10px"}}
        />
    </div>
  )
}

export default YoutubeEmbed