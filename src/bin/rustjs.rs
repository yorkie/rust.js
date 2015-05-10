#![cfg_attr(unix, feature(fs_ext))]

#[macro_use]
extern crate log
extern crate env_logger;

use std::io;
use std::fs;
use std::process::Command;

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
  env_logger::init().unwrap();
  
}