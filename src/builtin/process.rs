
use util::v8::{Object, String};

pub fn SetupProcess(process: Object) -> Object {
  process.Set(String::NewFromUtf8("title"), String::NewFromUtf8("rustjs"));
  process.Set(String::NewFromUtf8("version"), String::NewFromUtf8("1.0.0"));
  process
}
