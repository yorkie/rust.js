'use strict';

const assert = require('assert');
const url = require('url');

console.log(url.parse('https://git:123@github.com/url?foo=bar&loop=true#123'));