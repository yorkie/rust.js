
var net = require('net');

println('group id: ' + process.getgid());
println('process id: ' + process.getpid());
println(net);

process.exit(1);

