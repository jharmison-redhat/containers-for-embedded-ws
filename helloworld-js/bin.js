#!/usr/bin/env node
var server = require('./lib/server')

const listen = '127.0.0.1';
const port = 3000;

server.listen(port, listen, () => {
    console.log(`Server running at http://${listen}:${port}/`);
});
