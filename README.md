
Rust.js
-------------

NPM and Node.js compatible backend JavaScript platform, which is implemented in Rust.

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

Working modules
--------------

- [`assert`](src/builtin/assert.rs) This module is used for writing unit tests for your applications.
- [`buffer`](src/builtin/buffer.rs) Pure JavaScript is Unicode friendly but not nice to binary data.
- [`events`](src/builtin/events.js) Many objects in rust.js emit events.
- [`fs`](src/builtin/fs.rs) File I/O is provided by simple wrappers around standard POSIX functions.
- [`os`](src/builtin/os.rs) Provides a few basic operating-system related utility functions.
- [`path`](src/builtin/path.rs) This module contains utilities for handling and transforming file paths.
- [`util`](src/builtin/util.js) These functions are in the module 'util'.

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
