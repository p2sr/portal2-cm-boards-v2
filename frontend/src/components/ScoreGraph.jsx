import {BarChart} from "@mui/x-charts"

const ScoreGraph = props => {
    var columns = []
    var data = []
    var days = 7
    for(let i = 1; i < days + 1; i++){
        columns.push(props.graphData[days + 1 - i].date)
        data.push(props.graphData[days + 1 - i].count)
    }
    return <div flexDirection="column" justifyContent="center">
        <BarChart
            xAxis={[
                {
                id: 'barCategories',
                data: columns,
                scaleType: 'band',
                },
            ]}
            series={[   
                {
                data: data
                },
            ]}
            height={250}
        >
        </BarChart>
    </div>
}

export default ScoreGraph