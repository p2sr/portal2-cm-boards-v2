import { Box, useTheme } from "@mui/material"
import { tokens } from "../theme"
import SPMapScoreEntry from "./SPMapScoreEntry";

const SPMapScoreEntries = props => {
    const theme = useTheme();
    const colors = tokens(theme.palette.mode);

    var index = 0

    const mapData = props.mapData[0];

    return <div flexDirection="column" justifyContent="flex-start">
        {
            mapData.map(submission => {
                index++
                return <Box
                display="flex"
                flexDirection="column"
                style={{
                    backgroundColor:
                      index % 2 === 0
                        ? colors.gray[700]
                        : colors.gray[600]
                  }}
                >
                    <SPMapScoreEntry submission={submission}/>
                </Box>
            })
        }
    </div>
}

export default SPMapScoreEntries