'use strict';

var fs = require('fs');
var assert = require('assert');

// fs.stat
var stat = fs.stat('./makefile');
assert(typeof stat.dev === 'number');
assert(typeof stat.ino === 'number');

// fs.readdir
assert(Array.isArray(fs.readdir('./')));

// // fs.readFile
var dat = fs.readFile('./makefile');
assert(typeof dat === 'string');
