import React from 'react'

const YoutubeEmbed = ({embedId}) => {
  return (
    <div style={{padding:"15px"}}>
        <iframe
        width="534"
        height="300"
        src={`https://www.youtube.com/embed/${embedId}`}
        allowFullScreen
        frameBorder="none"
        title='YouTube embed'
        style={{border:'2px solid #3F414580', borderRadius:"10px"}}
        />
    </div>
  )
}

export default YoutubeEmbed