#[deny(unused)]

extern crate rustjs;
extern crate env_logger;
extern crate clap;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};
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
  env_logger::init().unwrap();
  let matches = App::new("rustjs")
    .version("0.1.0")
    .author("Yorkie Liu <yorkiefixer@gmail.com>")
    .about("The platform lets you can work with Rust, C, C++ and JavaScript compatible with NPM and Cargo")
    .arg(
      Arg::with_name("INPUT")
        .help("main script file")
        .required(true)
        .index(1)
    )
    .get_matches();

  let mut fd = match File::open(
    matches.value_of("INPUT").unwrap()) {
    Ok(fd) => fd,
    Err(..)  => panic!("room"),
  };
  let mut source = String::new();
  fd.read_to_string(&mut source);
  V8::Runtime(source.as_bytes());
}
