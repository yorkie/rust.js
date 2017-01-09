'use strict';

println('group id: ' + process.getgid());
println('process id: ' + process.getpid());
println('environment variables:');
println(Object.keys(process.env));

process.exit(1);
