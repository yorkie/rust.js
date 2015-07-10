
Rust.js
-------------

NPM and Node.js compatible backend JavaScript platform, which is implemented in Rust.

Build
-------------

Prerequisites:

- Rust 1.1.0
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

- [`path`](src/builtin/path.rs) This module contains utilities for handling and transforming file paths.
- [`fs`](src/builtin/fs.rs) File I/O is provided by simple wrappers around standard POSIX functions.
- [`os`](src/builtin/os.rs) Provides a few basic operating-system related utility functions.
- [`events`](src/builtin/events.js) Many objects in rust.js emit events.
- [`util`](src/builtin/util.js) These functions are in the module 'util'.
- [`buffer`](src/builtin/buffer.rs) Pure JavaScript is Unicode friendly but not nice to binary data.

License
-------------
MIT
