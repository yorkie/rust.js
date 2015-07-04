#![allow(non_snake_case)]

use std::env;
use util::v8;

extern {
  fn utils_is_big_endian() -> bool;
}

extern fn tmpdir(arguments: v8::FunctionCallbackInfo) {
  let dir = env::temp_dir();
  let val = v8::String::NewFromUtf8(dir.to_str().unwrap());
  arguments.GetReturnValue().Set(val);
}

extern fn homedir(arguments: v8::FunctionCallbackInfo) {
  let dir = env::home_dir();
  let val = v8::String::NewFromUtf8(dir.unwrap().to_str().unwrap());
  arguments.GetReturnValue().Set(val);
}

extern fn endianness(arguments: v8::FunctionCallbackInfo) {
  let isBig: bool;
  unsafe {
    isBig = utils_is_big_endian();
  }
  if isBig == true {
    arguments.GetReturnValue().Set(v8::String::NewFromUtf8("BE"));
  } else {
    arguments.GetReturnValue().Set(v8::String::NewFromUtf8("BE"));
  }
}

extern fn ostype(arguments: v8::FunctionCallbackInfo) {
  let val = env::consts::FAMILY;
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(val));
}

extern fn platform(arguments: v8::FunctionCallbackInfo) {
  let val = env::consts::OS;
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(val));
}

extern fn arch(arguments: v8::FunctionCallbackInfo) {
  let val = env::consts::ARCH;
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(val));
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("tmpdir"), 
    v8::FunctionTemplate::New(tmpdir).GetFunction());
  exports.Set(v8::String::NewFromUtf8("homedir"), 
    v8::FunctionTemplate::New(homedir).GetFunction());
  exports.Set(v8::String::NewFromUtf8("endianness"),
    v8::FunctionTemplate::New(endianness).GetFunction());
  exports.Set(v8::String::NewFromUtf8("type"),
    v8::FunctionTemplate::New(ostype).GetFunction());
  exports.Set(v8::String::NewFromUtf8("platform"),
    v8::FunctionTemplate::New(platform).GetFunction());
  exports.Set(v8::String::NewFromUtf8("arch"),
    v8::FunctionTemplate::New(arch).GetFunction());
  return exports;
}
