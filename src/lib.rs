#![allow(non_snake_case)]

extern crate libc;

use std::mem;

pub use util::cmd::Commander;
pub mod util;

extern {
  fn v8_free_platform() -> bool;
  fn v8_initialize_platform() -> bool;
  fn v8_initialize() -> bool;
  fn v8_dispose() -> bool;
  fn v8_set_array_buffer_allocator() -> bool;

  fn v8_locker_is_locked() -> bool;
  fn v8_locker_is_active() -> bool;
  fn v8_locker(closure: extern fn());

  fn v8_handle_scope(closure: extern fn());

  fn v8_isolate_new();
  fn v8_isolate_dispose();
  fn v8_isolate_enter();
  fn v8_isolate_exit();

  fn v8_context_new();
  fn v8_context_enter();
  fn v8_context_exit();
  fn v8_context_global() -> Object;
  fn v8_context_scope(closure: extern fn());

  fn v8_script_compile(source: &[u8]) -> Script;
  fn v8_script_run(this: &Script);

  fn v8_string_new_from_utf8(data: *const u8) -> String;
  fn v8_string_empty(this: &String) -> String;

  fn v8_object_new() -> Object;
  fn v8_object_get(this: &Object, key: &Value) -> Value;
  fn v8_object_set(this: &Object, key: &Value, val: &Value) -> bool;

  fn v8_function_tmpl_new() -> FunctionTemplate;
  fn v8_function_tmpl_set_class_name(this: &FunctionTemplate, name: &[u8]);
  fn v8_function_tmpl_new_instance(this: &FunctionTemplate) -> Object;

}

pub trait IndexT {
  fn get(&self, object: &Object) -> Value;
  fn set(&self, object: &Object, value: &Value) -> bool;
}

pub trait ValueT {
  fn as_val(&self) -> &Value;
}

macro_rules! value_method(
  ($ty:ident) => (
    impl IndexT for $ty {
      fn get(&self, object: &Object) -> Value {
        unsafe { 
          v8_object_get(object, self.as_val()) 
        }
      }
      fn set(&self, object: &Object, value: &Value) -> bool {
        unsafe { 
          v8_object_set(object, self.as_val(), value)
        }
      }
    }
    impl ValueT for $ty {
      fn as_val(&self) -> &Value {
        unsafe { mem::transmute(self) }
      }
    }
  )
);

#[repr(C)]
pub struct Locker(*mut u8);
impl Locker {
  pub fn IsLocked() -> bool {
    unsafe { v8_locker_is_locked() }
  }
  pub fn IsActive() -> bool {
    unsafe { v8_locker_is_active() }
  }
}
pub fn with_locker(closure: extern fn()) {
  unsafe {
    v8_locker(closure);
  }
}

#[repr(C)]
pub struct HandleScope(*mut u8);
pub fn with_handle_scope(closure: extern fn()) {
  unsafe { 
    v8_handle_scope(closure);
  }
}

#[repr(C)]
pub struct Isolate(*mut Isolate);
pub fn with_isolate_scope<T>(closure: &Fn() -> T) -> T {
  V8::EnterIsolate();
  let rval = closure();
  V8::ExitIsolate();
  rval
}

#[repr(C)]
pub struct Context(*mut Context);
impl Context {
  pub fn New() {
    unsafe { v8_context_new() }
  }
  pub fn Enter() {
    unsafe { v8_context_enter() }
  }
  pub fn Exit() {
    unsafe { v8_context_exit() }
  }
  pub fn Global() -> Object {
    unsafe { v8_context_global() }
  }
}
pub fn with_context_scope(closure: extern fn()) {
  unsafe {
    v8_context_scope(closure);
  }
}

#[repr(C)]
pub struct Script(*mut *mut Script);
impl Script {
  pub fn Compile(data: &[u8]) -> Script {
    unsafe { v8_script_compile(data) }
  }
  pub fn Run(&self) {
    unsafe { v8_script_run(self) }
  }
}

#[repr(C)]
pub struct Value(*mut *mut Value);
value_method!(Value);

#[repr(C)]
pub struct String(*mut *mut String);
value_method!(String);

impl String {
  pub fn NewFromUtf8(data: &str) -> String {
    unsafe { v8_string_new_from_utf8(data.as_ptr()) }
  }
  pub fn Empty(&self) -> String {
    unsafe { v8_string_empty(self) }
  }
}

#[repr(C)]
pub struct Object(*mut *mut Object);
value_method!(Object);

impl Object {
  pub fn New() -> Object {
    unsafe { v8_object_new() }
  }
  pub fn Get<K:IndexT>(&self, key: K) -> Value {
    key.get(self)
  }
  pub fn Set<K:IndexT, V:ValueT>(&self, key: K, value: V) -> bool {
    key.set(self, value.as_val())
  }
}

#[repr(C)]
pub struct FunctionTemplate(*mut *mut FunctionTemplate);
impl FunctionTemplate {
  pub fn New() -> FunctionTemplate {
    unsafe { v8_function_tmpl_new() }
  }
  pub fn SetClassName(&self, name: &[u8]) {
    unsafe { v8_function_tmpl_set_class_name(self, name) }
  }
  pub fn NewInstance(&self) -> Object {
    unsafe { v8_function_tmpl_new_instance(self) }
  }
}

#[repr(C)]
pub struct V8(*mut V8);
impl V8 {
  pub fn Runtime() -> i32 {
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
      let mut process = String::NewFromUtf8("process_str");
      let mut global = Context::Global();
      global.Set(String::NewFromUtf8("process"), process);

      let source = Commander::GetSource();
      let mut script = Script::Compile(source.as_bytes());
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
  pub fn UnInitializePlatform() -> bool {
    unsafe { v8_free_platform() }
  }
  pub fn InitializePlatform() -> bool {
    unsafe { v8_initialize_platform() }
  }
  pub fn Initialize() -> bool {
    unsafe { v8_initialize() }
  }
  pub fn Dispose() -> bool {
    unsafe { v8_dispose() }
  }
  pub fn SetArrayBufferAllocator() -> bool {
    unsafe { v8_set_array_buffer_allocator() }
  }
  pub fn NewIsolate() {
    unsafe { v8_isolate_new() }
  }
  pub fn DisposeIsolate() {
    unsafe { v8_isolate_dispose() }
  }
  pub fn EnterIsolate() {
    unsafe { v8_isolate_enter() }
  }
  pub fn ExitIsolate() {
    unsafe { v8_isolate_exit() }
  }
}
