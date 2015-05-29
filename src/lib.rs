#![allow(non_snake_case)]

pub mod util;
pub mod builtin;

use util::cmd::Commander;
use util::module::ModulesHeap;
use util::v8::{
  V8,
  Script,
  Context,
  String,
  Object,
  FunctionCallbackInfo,
  FunctionTemplate,
  with_isolate_scope,
  with_context_scope,
  with_handle_scope,
  with_locker
};

extern fn println(arguments: FunctionCallbackInfo) {
  let val = arguments.At(0).ToString();
  println!("{}", val.as_string());
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
    let process = Object::New();
    let global = Context::Global();
    let mut modules = ModulesHeap::New();

    modules.binding("buffer", builtin::buffer::Init());
    modules.binding("console", builtin::console::Init());
    modules.binding("crypto", builtin::crypto::Init());
    modules.binding("datagram", builtin::datagram::Init());
    modules.binding("dns", builtin::dns::Init());
    modules.binding("fs", builtin::fs::Init());
    modules.binding("http", builtin::http::Init());
    modules.binding("https", builtin::https::Init());
    modules.binding("net", builtin::net::Init());
    modules.binding("os", builtin::os::Init());
    modules.binding("path", builtin::path::Init());
    modules.binding("querystring", builtin::path::Init());
    modules.binding("readline", builtin::readline::Init());
    modules.binding("repl", builtin::repl::Init());
    modules.binding("tls", builtin::tls::Init());
    modules.binding("url", builtin::url::Init());

    global.Set(String::NewFromUtf8("process"), builtin::process::Setup(process));
    global.Set(String::NewFromUtf8("require"), builtin::module::Setup(modules));
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
