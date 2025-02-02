import { useState } from "react";
import { useEffect } from "react";
import {LineChart} from "@mui/x-charts"
import {format} from "date-fns";

const WRHistory = props => {

    const [wrHistory, setWrHistory] = useState([]);
    const [loading, setLoading] = useState(true);

    const levelId = props.levelId;
    
    //fetching changelog data on first component load
    useEffect(() => {
        const fetchData = async () => {
            try {
                const wrHistoryResponse = await Promise.all([
                    fetch(`http://localhost:8080/api/v1/changelog?chamber=${levelId}&wr_gain=true`).then(response => {
                        if (!response.ok) {
                            throw new Error('WR History response not OK');
                        }
                        return response.json();
                    })
                ]);
                setWrHistory(wrHistoryResponse);
                setLoading(false);
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };
        fetchData();
    }, []);

    var filteredHistory = [];
    var wrDataPoints = [];
    var wrTimePoints = [];

    if (!loading) {
        filteredHistory = wrHistory[0].filter((obj) => !obj.banned)
        wrDataPoints = filteredHistory.map(obj => obj.score);
        wrTimePoints = filteredHistory.map(obj => new Date(obj.timestamp).getTime());
        console.log(filteredHistory)
    }

    return <div flexDirection="column" justifyContent="center">
        {loading ? null :
            <LineChart
            xAxis={[{
                scaleType: "time",
                data: wrTimePoints,
            }]}
            series={[
                {
                data: wrDataPoints,
                },
            ]}
            slotProps={{
                tooltip: { trigger: "item" }, // Ensures tooltip follows the cursor
              }}
            width={500}
            height={250}
            />
        }
    </div>
}

export default WRHistory;