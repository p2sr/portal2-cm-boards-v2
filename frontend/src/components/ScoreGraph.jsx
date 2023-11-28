import {BarChart} from "@mui/x-charts"

const ScoreGraph = props => {
    var columns = []
    var data = []
    for(let i = 1; i < 6; i++){
        columns.push(props.graphData[i].date)
        data.push(props.graphData[i].count)
    }
    return <div flexDirection="column" justifyContent="flex-start">
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
        />
    </div>
}

export default ScoreGraph