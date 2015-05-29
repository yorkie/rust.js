
extern crate libc;

use std::process;
use util::v8::{Object, Number, String, FunctionCallbackInfo, FunctionTemplate};
use util::config;

use self::libc::funcs::posix88::unistd;

extern fn exit(arguments: FunctionCallbackInfo) {
  process::exit(arguments.At(0).Int32Value())
}

extern fn getgid(arguments: FunctionCallbackInfo) {
  arguments.GetReturnValue().SetWithUint32(
    unsafe { unistd::getgid() }
  );
}

extern fn getpid(arguments: FunctionCallbackInfo) {
  arguments.GetReturnValue().SetWithInt32(
    unsafe { unistd::getpid() }
  );
}

pub fn SetupProcess(process: Object) -> Object {
  // bind functions
  process.Set(String::NewFromUtf8("exit"), FunctionTemplate::New(exit).GetFunction());
  process.Set(String::NewFromUtf8("getgid"), FunctionTemplate::New(getgid).GetFunction());
  process.Set(String::NewFromUtf8("getpid"), FunctionTemplate::New(getpid).GetFunction());
  // bind consts
  process.Set(String::NewFromUtf8("title"), String::NewFromUtf8(config::NAME));
  process.Set(String::NewFromUtf8("version"), String::NewFromUtf8(config::VERSION));
  process
}
