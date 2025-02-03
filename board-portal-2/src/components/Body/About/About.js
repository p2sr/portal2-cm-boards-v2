import { div } from "@material-ui/core"
import React from "react"
import { ThemeContext } from "../../../App"
import "./about.css"
import { useStyles } from "./style"
/**
 * @name - About
 * @desc - Page for the "about" section. Contains formula for calculating
 *         top 200 times and a table containing an example top 200 times.
 * @author - Mitchell Baker
 * @date - 3/17/21
 * @version - 1.0
 * @param -
 * @return -
 */

const About = () => {
  const classes = useStyles()
  const themeContext = React.useContext(ThemeContext)
  return (
    <div className={classes.aboutBody}>
      <h1 className={classes.aboutHeader}>About</h1>
      <h1 className={classes.aboutSubHeader}>Rules & Console Commands</h1>
      <div className={classes.aboutText}>
        For rules and console commands, go <a>here</a>.
      </div>
      <h1 className={classes.aboutSubHeader}>Point Calculation</h1>
      <div className={classes.aboutText}>
        Currently, the top 200 times are tracked for each map. A player's points
        for a specific chamber is calculated as follows:
      </div>
      <table
        className={classes.fraction}
        style={{
          color: themeContext.theme.palette.text.primary
        }}>
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
          <td
            className={classes.upperLine}
            style={{
              textAlign: "center",
              color: themeContext.theme.palette.text.primary
            }}>
            200
          </td>
        </tr>
      </table>
      <div className={classes.aboutText}>This gives us the following:</div>
      <table
        className='pointTable table table-condensed'
        style={{
          marginTop: "20px",
          width: "160px",
          color: themeContext.theme.palette.text.primary
        }}>
        <tr>
          <th
            style={{
              color: themeContext.theme.palette.text.primary
            }}>
            Rank
          </th>
          <th
            style={{
              color: themeContext.theme.palette.text.primary
            }}>
            Points
          </th>
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
      <h1 className={classes.aboutSubHeader}>Team</h1>
      <div className={classes.aboutTeamList}>
        <ul>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
          <li>Randy Savage</li>
        </ul>
      </div>
      <h1 className={classes.aboutSubHeader}>API</h1>
      <div className={classes.aboutText}>
        A JSON representation of the current leaderboard status can be retrieved
        by appending '/json' to any page URL, i.e.
        board.iverb.me/chamber/47458/json or board.iverb.me/profile/iVerb/json.
      </div>
      <h1 className={classes.aboutSubHeader}>Libraries</h1>
      <div>
        <table
          className='pointTable table table-condensed'
          style={{
            marginTop: "20px",
            width: "160px",
            color: themeContext.theme.palette.text.primary
          }}>
          <tr>
            <th
              style={{
                color: themeContext.theme.palette.text.primary
              }}>
              Name
            </th>
            <th
              style={{
                color: themeContext.theme.palette.text.primary
              }}>
              Language
            </th>
          </tr>
          <tr>
            <td>Portal2Boards.Net</td>
            <td>C#</td>
          </tr>
        </table>
      </div>
      <h1 className={classes.aboutSubHeader}>Hosting</h1>
      <div className={classes.aboutText}>
        This site is currently hosted on FastComet. Demos are stored using
      </div>
      <div className={classes.aboutText}>
        Google Drive. The folder containing all demos can be found here.
      </div>
      <h1 className={classes.aboutSubHeader}>Feature Suggestions</h1>
      <div className={classes.aboutText}>
        Forward any feature suggestions you may have to NeKz or iVerb.
      </div>
    </div>
  )
}

export default About
