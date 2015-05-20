#[deny(unused)]

extern crate clap;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use self::clap::{Arg, App};

pub struct Commander;
impl Commander {
  
  pub fn GetSource() -> String {
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
    let mut fd = match File::open(matches.value_of("INPUT").unwrap()) {
      Ok(fd) => fd,
      Err(..)  => panic!("room"),
    };
    let mut source = String::new();
    fd.read_to_string(&mut source);
    return source;
  }

}


