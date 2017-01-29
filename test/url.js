'use strict';

const assert = require('assert');
const url = require('url');

let obj = url.parse('https://git:123@github.com/url?foo=bar&loop=true#123');
assert.equal(obj.protocol, 'https');
assert.equal(obj.username, 'git');
assert.equal(obj.password, '123');
assert.equal(obj.hostname, 'github.com');
assert.equal(obj.port, 443);
assert.equal(obj.pathname, '/url');
assert.equal(obj.query, 'foo=bar&loop=true');
assert.equal(obj.hash, '123');

let str = url.format({
  protocol: 'http',
  port: 3000,
  hostname: 'github.com',
  pathname: '/issues',
  query: 'foo=bar',
  hash: 'loop'
});
assert.equal(str, 
  'http://github.com:3000/issues?foo=bar#loop');

console.log('test/url.js is done');
