
extern crate rustjs;
extern crate env_logger;

use rustjs::util::v8::V8;

fn main() {
  env_logger::init().unwrap();
  V8::Runtime();
}
