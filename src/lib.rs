#![allow(non_snake_case)]

pub mod util;
pub mod builtin;

use builtin::process::SetupProcess;
use util::cmd::Commander;
use util::v8::{
  V8,
  Script,
  Context,
  // Value,
  String,
  Object,
  // Function,
  // FunctionCallback,
  FunctionCallbackInfo,
  FunctionTemplate,
  with_isolate_scope,
  with_context_scope,
  with_handle_scope,
  with_locker
};

extern fn println(arguments: FunctionCallbackInfo) {
  let val = arguments.At(0).ToString();
  // println!("{:?}", val);
}

pub fn new_instance() -> i32 {
  extern fn on_locked() {
    with_isolate_scope(&|| {
      with_handle_scope(on_handle_scoped);
    });
  }
  extern fn on_handle_scoped() {
    Context::New();
    with_context_scope(on_context_scoped);
  }
  extern fn on_context_scoped() {
    let mut process = Object::New();
    let global = Context::Global();

    process = SetupProcess(process);
    global.Set(String::NewFromUtf8("process"), process);
    global.Set(String::NewFromUtf8("println"), FunctionTemplate::New(println).GetFunction());

    let source = Commander::GetSource();
    let script = Script::Compile(source.as_bytes());
    script.Run();
  }

  let code :i32 = 1;
  V8::InitializePlatform();
  V8::Initialize();
  V8::SetArrayBufferAllocator();
  V8::NewIsolate();
  with_locker(on_locked);
  V8::DisposeIsolate();
  V8::Dispose();
  V8::UnInitializePlatform();
  return code;
}
