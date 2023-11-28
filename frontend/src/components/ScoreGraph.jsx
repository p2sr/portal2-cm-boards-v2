import {BarChart} from "@mui/x-charts"

const ScoreGraph = props => {
    var columns = []
    var data = []
    console.log(props)
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
            height={300}
        />
    </div>
}

export default ScoreGraph