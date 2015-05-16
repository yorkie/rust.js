#[deny(unused)]

extern crate rustjs;
extern crate env_logger;

use std::env;
use std::fs;
use std::io;

use rustjs::V8;

const USAGE: &'static str = "
Rust.js lets you:

1. run NPM-compatible modules
2. run JavaScript codes as front-end language
3. bind the eco-system of NPM and Cargo

Usage:
    rustj <command> [<args>...]
    rustj [options]
Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output
";

fn main() {
  println!("main function from rust");
  env_logger::init().unwrap();
  return V8::Runtime();
}
