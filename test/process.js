'use strict';

const thread = require('thread');

console.log('group id: ' + process.getgid());
console.log('process id: ' + process.getpid());
console.log('environment variables:');
console.log(JSON.stringify(process.env, null, 2));
