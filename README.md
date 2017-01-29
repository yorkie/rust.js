
Rust.js
-------------

Run your JavaScript apps backed by Rust.

Build
-------------

Prerequisites:

- Rust 1.10.0+
- Python 2.6 or 2.7
- GNU Make 3.81 or newer

```
$ ./configure
$ make
```

Usage
-------------

```
rustjs 0.1.0
Yorkie Liu <yorkiefixer@gmail.com>
The platform lets you can work with Rust, C, C++ and JavaScript compatible with NPM and Cargo

USAGE:
  rustjs <INPUT> [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

POSITIONAL ARGUMENTS:
    INPUT      main script file
```

API References
--------------

- [`assert`](src/builtin/assert.rs) This module is used for writing unit tests for your applications.
  - `assert.equal()`
  - `assert.notEqual()`
- [`console`](src/builtin/console.rs) Console object.
  - `console.log()`
  - `console.info()`
  - `console.warn()`
  - `console.error()`
- [`events`](src/builtin/events.js) Many objects in rust.js emit events.
  - `events.EventEmitter`
    - `.on(name, listener)`
    - `.once(name, listener)`
    - `.removeAllEventListeners()` remove all registered listeners on this emitter.
- [`fs`](src/builtin/fs.rs) File I/O is provided by simple wrappers around standard POSIX functions.
  - `fs.rename()` rename the file.
  - `fs.rmdir()` remove the dir.
  - `fs.mkdir()` create the dir.
  - `fs.stat()` get the stat of the given pathname.
  - `fs.readdir()` read the directory.
  - `fs.readFile()` read the file of the given pathname.
  - `fs.writeFile()` write the file of the given pathname and bytes/string.
- [`os`](src/builtin/os.rs) Provides a few basic operating-system related utility functions.
  - `fs.tmpdir()`
  - `fs.homedir()`
  - `fs.endianness()`
  - `fs.type()`
  - `fs.platform()`
  - `fs.arch()`
- [`path`](src/builtin/path.rs) This module contains utilities for handling and transforming file paths.
  - `path.normalize()` normalize the path string.
  - `path.isAbsolute()` return if a path is in absolute.
  - `path.dirname()` return the directory name of the path.
  - `path.basename()` return the basename of the path.
  - `path.extname()` return the ext name.
- [`url`](src/builtin/url.rs) The URL parser/formatter based on [servo/rust-url].
  - `url.parse()`: parse a url string.
  - `url.format()`: generate a string by url options.
- [`util`](src/builtin/util.rs) These functions are in the module 'util'.
  - `util.isArray()`
  - `util.isFunction()`
  - `util.isBoolean()`
  - `util.isNull()`
  - `util.isNullOrUndefined()`
  - `util.isNumber()`
  - `util.isString()`
  - `util.isUndefined()`
  - `util.isRegExp()`
  - `util.isObject()`
  - `util.isDate()`
  - `util.isError()`
  - `util.isPrimitive()`
  - `util.inherits()`

Development
-------------

Clone the repository

```sh
$ git clone git@github.com:yorkie/rust.js.git
```

Pull submodules

```sh
$ git submodule update --init --recursive
```

Build dependencies

```sh
$ ./configure && make
```


License
-------------
MIT

[servo/rust-url]: https://github.com/servo/rust-url
