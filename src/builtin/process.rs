
use std::process;

use util::v8::{Object, String, FunctionCallbackInfo, FunctionTemplate};
use util::config;

extern fn exit(arguments: FunctionCallbackInfo) {
  process::exit(arguments.At(0).Int32Value())
}

pub fn SetupProcess(process: Object) -> Object {
  process.Set(String::NewFromUtf8("title"), String::NewFromUtf8(config::NAME));
  process.Set(String::NewFromUtf8("version"), String::NewFromUtf8(config::VERSION));
  process.Set(String::NewFromUtf8("exit"), FunctionTemplate::New(exit).GetFunction());
  process
}
