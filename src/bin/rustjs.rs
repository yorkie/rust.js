
extern crate rustjs;
extern crate env_logger;

use rustjs::V8;

fn main() {
  env_logger::init().unwrap();
  V8::Runtime();
}
