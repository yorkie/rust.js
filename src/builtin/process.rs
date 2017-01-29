
extern crate libc;

use std::env;
use std::process;
use util::v8;
use util::config;

use self::libc::funcs::posix88::unistd;

extern fn exit(arguments: v8::FunctionCallbackInfo) {
  process::exit(arguments.At(0).Int32Value())
}

extern fn getgid(arguments: v8::FunctionCallbackInfo) {
  arguments.GetReturnValue().SetWithUint32(
    unsafe { unistd::getgid() }
  );
}

extern fn getpid(arguments: v8::FunctionCallbackInfo) {
  arguments.GetReturnValue().SetWithInt32(
    unsafe { unistd::getpid() }
  );
}

pub fn Init() -> v8::Object {
  let Process = v8::Object::New();
  // bind functions
  Process.Set(v8::String::NewFromUtf8("exit"), v8::FunctionTemplate::New(exit).GetFunction());
  Process.Set(v8::String::NewFromUtf8("getgid"), v8::FunctionTemplate::New(getgid).GetFunction());
  Process.Set(v8::String::NewFromUtf8("getpid"), v8::FunctionTemplate::New(getpid).GetFunction());
  // bind consts
  Process.Set(v8::String::NewFromUtf8("title"), v8::String::NewFromUtf8(config::NAME));
  Process.Set(v8::String::NewFromUtf8("version"), v8::String::NewFromUtf8(config::VERSION));
  // set process.env
  let envObject = v8::Object::New();
  for (key, val) in env::vars() {
    envObject.Set(v8::String::NewFromUtf8(&*key), v8::String::NewFromUtf8(&*val));
  }
  Process.Set(v8::String::NewFromUtf8("env"), envObject);
  return Process;
}
