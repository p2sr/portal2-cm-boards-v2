import { div } from "@material-ui/core"
import "./about.css"

const About = () => {
  return (
    <div className='about-body'>
      <h1 className='about-header'>About</h1>
      <h1 className='about-sub-header'>Rules & Console Commands</h1>
      <div className='about-text'>
        For rules and console commands, go <a>here</a>.
      </div>
      <h1 className='about-sub-header'>Point Calculation</h1>
      <div className='about-text'>
        Currently, the top 200 times are tracked for each map. A player's points
        for a specific chamber is calculated as follows:
      </div>
      <table className='fraction'>
        <tr>
          <td rowSpan='2' nowrap='nowrap'>
            max[1,&nbsp;
          </td>
          <td nowrap='nowrap'>
            (200 - (rank - 1))<sup>2</sup>
          </td>
          <td rowSpan='2' nowrap='nowrap'>
            ]
          </td>
        </tr>
        <tr>
          <td className='upper-line' style={{ textAlign: "center" }}>
            200
          </td>
        </tr>
      </table>
      <div className='about-text'>This gives us the following:</div>
      <table
        className='pointTable table table-condensed'
        style={{ marginTop: "20px", width: "160px" }}>
        <tr>
          <th>Rank</th>
          <th>Points</th>
        </tr>
        <tr>
          <td>1</td>
          <td>200</td>
        </tr>
        <tr>
          <td>5</td>
          <td>193</td>
        </tr>
        <tr>
          <td>10</td>
          <td>183</td>
        </tr>
        <tr>
          <td>20</td>
          <td>164</td>
        </tr>
        <tr>
          <td>50</td>
          <td>115</td>
        </tr>
        <tr>
          <td>100</td>
          <td>52</td>
        </tr>
        <tr>
          <td>150</td>
          <td>14</td>
        </tr>
        <tr>
          <td>200</td>
          <td>1</td>
        </tr>
      </table>
      <h1 className='about-sub-header'>Team</h1>
      <div className='about-team-list'>
        <ul>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
        </ul>
      </div>
      <h1 className='about-sub-header'>API</h1>
      <div className='about-text'>
        A JSON representation of the current leaderboard status can be retrieved
        by appending '/json' to any page URL, i.e.
        board.iverb.me/chamber/47458/json or board.iverb.me/profile/iVerb/json.
      </div>
      <h1 className='about-sub-header'>Libraries</h1>
      <div>
        <table
          className='pointTable table table-condensed'
          style={{ marginTop: "20px", width: "160px" }}>
          <tr>
            <th>Name</th>
            <th>Language</th>
          </tr>
          <tr>
            <td>Portal2Boards.Net</td>
            <td>C#</td>
          </tr>
        </table>
      </div>
      <h1 className='about-sub-header'>Hosting</h1>
      <div className='about-text'>
        This site is currently hosted on FastComet. Demos are stored using
      </div>
      <div className='about-text'>
        Google Drive. The folder containg all demos can be found here.
      </div>
      <h1 className='about-sub-header'>Feature Suggestions</h1>
      <div className='about-text'>
        Forward any feature suggestions you may have to NeKz or iVerb.
      </div>
    </div>
  )
}

export default About
