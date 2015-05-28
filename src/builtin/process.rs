
extern crate libc;

use std::process;
use util::v8::{Object, Number, String, FunctionCallbackInfo, FunctionTemplate};
use util::config;

use self::libc::funcs::posix88::unistd;

extern fn exit(arguments: FunctionCallbackInfo) {
  process::exit(arguments.At(0).Int32Value())
}

extern fn getgid(arguments: FunctionCallbackInfo) {
  unsafe {
    let gid = unistd::getgid();
    println!("{:?}", gid);
  }
}

pub fn SetupProcess(process: Object) -> Object {
  process.Set(String::NewFromUtf8("exit"), FunctionTemplate::New(exit).GetFunction());
  process.Set(String::NewFromUtf8("getgid"), FunctionTemplate::New(getgid).GetFunction());
  process.Set(String::NewFromUtf8("title"), String::NewFromUtf8(config::NAME));
  process.Set(String::NewFromUtf8("version"), String::NewFromUtf8(config::VERSION));
  process
}
