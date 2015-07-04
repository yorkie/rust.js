

var fs = require('fs');
var stat = fs.stat('./makefile');
println(JSON.stringify(stat));
println(JSON.stringify(fs.readdir('./')));