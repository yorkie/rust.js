
use util::v8::{Object, String};

pub fn SetupProcess(process: Object) -> Object {
  process.Set(String::NewFromUtf8("test1"), String::NewFromUtf8("val"));
  process
}
