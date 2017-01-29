
use util::v8;
use util::v8::ValueT;
use std::io::{self, Write};

extern fn info(arguments: v8::FunctionCallbackInfo) {
  let global = v8::Context::Global();
  let firstArg = arguments.At(0);
  let msg;

  if firstArg.IsString() || firstArg.IsFunction() {
    msg = firstArg.ToString();
  } else {
    let JSON = global.Get(v8::String::NewFromUtf8("JSON")).ToObject();
    let stringify: v8::Function = v8::Function::Cast(
      &JSON.Get(v8::String::NewFromUtf8("stringify")));
    let space = v8::Number::NewFromInt32(0);
    let indent = v8::Number::NewFromInt32(2);
    msg = stringify.Call(JSON,
      &[&firstArg, &space.as_val(), &indent.as_val()]).ToString();
  }

  let stdout = io::stdout();
  let mut handle = stdout.lock();
  handle.write_fmt(format_args!("{}\n", msg.as_string()));
}

extern fn warn(arguments: v8::FunctionCallbackInfo) {
  let msg = arguments.At(0).ToString();
  let stderr = io::stderr();
  let mut handle = stderr.lock();
  handle.write_fmt(format_args!("{}\n", msg.as_string()));
}

pub fn Init() -> v8::Object {
  let Console = v8::Object::New();
  Console.Set(v8::String::NewFromUtf8("log"), v8::FunctionTemplate::New(info).GetFunction());
  Console.Set(v8::String::NewFromUtf8("info"), v8::FunctionTemplate::New(info).GetFunction());
  Console.Set(v8::String::NewFromUtf8("warn"), v8::FunctionTemplate::New(warn).GetFunction());
  Console.Set(v8::String::NewFromUtf8("error"), v8::FunctionTemplate::New(warn).GetFunction());
  return Console;
}
