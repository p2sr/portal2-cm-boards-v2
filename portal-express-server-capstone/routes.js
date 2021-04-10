const express = require('express');
const router = express.Router();

router.get('/sp', (req, res) => {
    const stats = require('./dump/sp_preview.json');

    return res.status(200).json(stats);
});

router.get('/coop', (req, res) => {
    const stats = require('./dump/coop_preview.json');

    return res.status(200).json(stats);
});

router.get('/maps/sp/:map_id', (req, res) => {
    const stats = require(`./api/sp/${req.params.map_id}.json`);

    return res.status(200).json(stats);
});

router.get('/maps/coop/:map_id', (req, res) => {
    const stats = require(`./api/coop/${req.params.map_id}.json`);

    return res.status(200).json(stats);
});

module.exports = router;