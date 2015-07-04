#![allow(non_snake_case)]

#[macro_use]
pub mod util;
pub mod builtin;

use util::cmd::Commander;
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
    let modules = v8::Object::New();

    modules.Set(v8::String::NewFromUtf8("buffer"), builtin::buffer::Init());
    modules.Set(v8::String::NewFromUtf8("console"), builtin::console::Init());
    modules.Set(v8::String::NewFromUtf8("crypto"), builtin::crypto::Init());
    modules.Set(v8::String::NewFromUtf8("datagram"), builtin::datagram::Init());
    modules.Set(v8::String::NewFromUtf8("dns"), builtin::dns::Init());
    modules.Set(v8::String::NewFromUtf8("fs"), builtin::fs::Init());
    modules.Set(v8::String::NewFromUtf8("http"), builtin::http::Init());
    modules.Set(v8::String::NewFromUtf8("https"), builtin::https::Init());
    modules.Set(v8::String::NewFromUtf8("net"), builtin::net::Init());
    modules.Set(v8::String::NewFromUtf8("os"), builtin::os::Init());
    modules.Set(v8::String::NewFromUtf8("path"), builtin::path::Init());
    modules.Set(v8::String::NewFromUtf8("querystring"), builtin::path::Init());
    modules.Set(v8::String::NewFromUtf8("readline"), builtin::readline::Init());
    modules.Set(v8::String::NewFromUtf8("repl"), builtin::repl::Init());
    modules.Set(v8::String::NewFromUtf8("tls"), builtin::tls::Init());
    modules.Set(v8::String::NewFromUtf8("url"), builtin::url::Init());

    global.Set(v8::String::NewFromUtf8("_modules"), modules);
    global.Set(v8::String::NewFromUtf8("process"), builtin::process::Setup(process));
    global.Set(v8::String::NewFromUtf8("require"), builtin::module::Setup());
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
