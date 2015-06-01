use util::v8;
use std::collections::HashMap;

extern fn require(arguments: v8::FunctionCallbackInfo) {
  let name = arguments.At(0).ToString();
  let modules = v8::Context::Global().Get(v8::String::NewFromUtf8("_modules")).ToObject();
  arguments.GetReturnValue().Set(modules.Get(name));
}

pub fn Setup() -> v8::Function {
  v8::FunctionTemplate::New(require).GetFunction()
}
