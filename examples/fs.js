

var fs = require('fs');

// fs.stat
var stat = fs.stat('./makefile');
println(JSON.stringify(stat));

// fs.readdir
fs.readdir('./').forEach(function (p) {
  println(p)
});