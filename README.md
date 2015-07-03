
Rust.js
-------------

NPM and Node.js compatible backend JavaScript platform, which is implemented in Rust.

Build
-------------

Prerequisites:

- Rust 1.0.0
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

- [`path`](src/builtin/path.rs)
- [`fs`](src/builtin/fs.rs)


License
-------------
MIT
