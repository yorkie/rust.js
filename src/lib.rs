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
}

#[repr(C)]
pub struct Isolate(*mut Isolate);

#[repr(C)]
pub struct Context(*mut Context);

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
