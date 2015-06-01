

var path = require('path');

println(path.normalize('/foo/bar//baz/asdf/quux/..'));

println(path.isAbsolute('foo.txt'));
println(path.isAbsolute('/foo.txt'));

println(path.dirname('/foo/bar'));
println(path.dirname('/foo/bar/xxx'));

println(path.basename('/foo/bar'));
println(path.basename('/foo/bar.rs'));

println(path.extname('/foo/bar.rs'));

// path.sep
println('the path.sep = ' + path.sep);
