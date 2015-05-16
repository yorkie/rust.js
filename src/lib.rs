#![allow(non_snake_case)]
#![allow(non_uppercase_statics)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

extern {
  fn v8_initialize() -> bool;
}

#[repr(C)]
pub struct Isolate(*mut Isolate);

impl Isolate {
  pub fn Dispose(&mut self) {
    // TODO
  }
}

#[repr(C)]
pub struct V8(*mut V8);

impl V8 {
  pub fn Initialize() -> bool {
    unsafe { v8_initialize() }
  }
}
