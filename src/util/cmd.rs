#[deny(unused_must_use)]

extern crate clap;

use std::io::prelude::*;
use std::fs::File;

use self::clap::{Arg, App};
use util::config;

pub struct Commander;
impl Commander {
  
  pub fn GetSource() -> String {
    let matches = App::new(config::NAME)
    .version(config::VERSION)
    .author(config::AUTHOR)
    .about(config::DESCRIPTION)
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
    match fd.read_to_string(&mut source) {
      Ok(val) => println!("successfully read the source: {:?}", val),
      _ => println!("failed to read source")
    }
    return source;
  }

}
