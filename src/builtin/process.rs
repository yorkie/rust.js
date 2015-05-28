
use util::v8::{Object, String};
use util::config;

pub fn SetupProcess(process: Object) -> Object {
  process.Set(String::NewFromUtf8("title"), String::NewFromUtf8(config::NAME));
  process.Set(String::NewFromUtf8("version"), String::NewFromUtf8(config::VERSION));
  process
}
