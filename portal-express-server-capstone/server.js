const express = require('express');
const routes = require('./routes');
const cors = require('cors');
const app = express();
const port = process.env.PORT || 8080;

const cors_options = {
    origin: '*'
}

app.use(
    express.urlencoded({
        extended: false
    })
);

app.use(express.json());

app.use('/api', cors(cors_options), routes);
app.listen(port, () => console.log(`Server up and running on port ${port}`));