'use strict';

const buffer = require('buffer');
const stringBytes = new buffer.StringBytes();

console.log(buffer.StringBytes);
console.log(stringBytes.encode);
console.log(stringBytes.decode);
