const http = require('http');
const os = require("os");

const server = http.createServer((req, res) => {
    var hostname = os.hostname();
    console.log(`Greeted ${req.socket.localAddress} from ${hostname}`)
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end(`Hello, world, from ${hostname}!\n`);
});

module.exports = server;
