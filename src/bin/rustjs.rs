
extern crate rustjs;
extern crate env_logger;

fn main() {
  env_logger::init().unwrap();
  rustjs::new_instance();
}
