

var path = require('path');

console.log(path.normalize('/foo/bar//baz/asdf/quux/..'));

console.log(path.isAbsolute('foo.txt'));
console.log(path.isAbsolute('/foo.txt'));

console.log(path.dirname('/foo/bar'));
console.log(path.dirname('/foo/bar/xxx'));

console.log(path.basename('/foo/bar'));
console.log(path.basename('/foo/bar.rs'));

console.log(path.extname('/foo/bar.rs'));

// path.sep
console.log('the path.sep = ' + path.sep);
