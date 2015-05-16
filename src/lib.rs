#![allow(non_snake_case)]
#![allow(non_uppercase_statics)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

extern {
  fn v8_runtime(source: &[u8]) -> i16;
  fn v8_initialize() -> bool;
  fn v8_dispose() -> bool;
}

#[repr(C)]
pub struct V8(*mut V8);

impl V8 {
  pub fn Runtime(source: &[u8]) -> i16 {
    unsafe { v8_runtime(source) }
  }
  pub fn Initialize() -> bool {
    unsafe { v8_initialize() }
  }
  pub fn Dispose() -> bool {
    unsafe { v8_dispose() }
  }
}


