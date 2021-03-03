#!/usr/bin/env node
var server = require('./lib/server')
var args = process.argv.slice(2);

if (args.length == 0) {
    var listen = '127.0.0.1';
} else {
    var listen = args[0];
}
const port = 3000;

server.listen(port, listen, () => {
    console.log(`Server running at http://${listen}:${port}/`);
});
