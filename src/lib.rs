#![allow(non_snake_case)]

pub mod util;
pub mod builtin;

use util::cmd::Commander;
use util::module::ModulesHeap;
use util::v8;

extern fn println(arguments: v8::FunctionCallbackInfo) {
  let val = arguments.At(0).ToString();
  println!("{}", val.as_string());
}

pub fn new_instance() -> i32 {
  extern fn on_locked() {
    v8::with_isolate_scope(&|| {
      v8::with_handle_scope(on_handle_scoped);
    });
  }
  extern fn on_handle_scoped() {
    v8::Context::New();
    v8::with_context_scope(on_context_scoped);
  }
  extern fn on_context_scoped() {
    let process = v8::Object::New();
    let global = v8::Context::Global();
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

    global.Set(v8::String::NewFromUtf8("process"), builtin::process::Setup(process));
    global.Set(v8::String::NewFromUtf8("require"), builtin::module::Setup(modules));
    global.Set(v8::String::NewFromUtf8("println"), v8::FunctionTemplate::New(println).GetFunction());

    let source = Commander::GetSource();
    let script = v8::Script::Compile(source.as_bytes());
    script.Run();
  }

  let code :i32 = 1;
  v8::V8::InitializePlatform();
  v8::V8::Initialize();
  v8::V8::SetArrayBufferAllocator();
  v8::V8::NewIsolate();
  v8::with_locker(on_locked);
  v8::V8::DisposeIsolate();
  v8::V8::Dispose();
  v8::V8::UnInitializePlatform();
  return code;
}
