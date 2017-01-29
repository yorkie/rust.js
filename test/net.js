'use strict';

const net = require('net');
const c = new net.Socket('baidu.com:80');

console.log(net.Socket);
c.connect();
console.log(c);
