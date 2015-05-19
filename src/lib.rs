#![allow(non_snake_case)]
#![allow(non_uppercase_statics)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

extern {
  fn v8_runtime(source: &[u8]) -> i32;
  fn v8_free_platform() -> bool;
  fn v8_initialize_platform() -> bool;
  fn v8_initialize() -> bool;
  fn v8_dispose() -> bool;
  fn v8_set_array_buffer_allocator() -> bool;
  fn v8_isolate_new();
  fn v8_isolate_dispose();

  fn v8_locker_is_locked(isolate: Isolate) -> bool;
  fn v8_locker_is_active() -> bool;
  fn v8_locker_initialize(this: &mut Locker, isolate: Isolate);

  fn v8_script_compile(isolate: Isolate, source: String) -> Script;
  fn v8_script_run(this: Script);
}

#[repr(C)]
pub struct Locker(*mut u8);

impl Locker {
  pub fn IsLocked(isolate: Isolate) -> bool {
    unsafe { v8_locker_is_locked(isolate) }
  }
  pub fn IsActive() -> bool {
    unsafe { v8_locker_is_active() }
  }
}

pub fn with_locker<T>(isolate: Isolate, closure: fn() -> T) -> T {
  let mut this = Locker(ptr::null_mut());
  unsafe { v8_locker_initialize(&mut this, isolate) };
  
  let rval = closure();
  rval
}

#[repr(C)]
pub struct Isolate(*mut Isolate);

#[repr(C)]
pub struct Context(*mut Context);

#[repr(C)]
pub struct Script(*mut *mut Script);

impl Script {
  pub fn Compile(isolate: Isolate, data: String) -> Script {
    unsafe { v8_script_compile(isolate, data) }
  }
  pub fn Run(&mut self) {
    // copy self
    // unsafe { v8_script_run(*self) }
  }
}

#[repr(C)]
pub struct V8(*mut V8);

impl V8 {
  pub fn Runtime(source: &[u8]) -> i32 {
    let mut code :i32 = 1;
    V8::InitializePlatform();
    V8::Initialize();
    V8::SetArrayBufferAllocator();
    V8::NewIsolate();
    unsafe {
      code = v8_runtime(source);
    }
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
}
