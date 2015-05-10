#![allow(non_snake_case)]
#![allow(non_uppercase_statics)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

#[link(name="v8", kind="static")]
extern {
  fn _ZN2v82V810InitializeEv() -> bool;
  fn _ZN2v82V87DisposeEv() -> bool;
}

#[repr(C)]
pub struct V8(*mut V8);

impl V8 {
    pub fn Initialize() -> bool {
        unsafe { _ZN2v82V810InitializeEv() }
    }

    pub fn Dispose() -> bool {
        unsafe { _ZN2v82V87DisposeEv() }
    }
}
