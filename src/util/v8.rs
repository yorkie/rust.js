#![allow(non_snake_case)]

extern crate libc;

use std::mem;
use std::string;
use std::ffi::CString;
use std::ffi::CStr;

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

  fn v8_value_is_string(this: &Value) -> bool;
  fn v8_value_to_string(this: &Value) -> String;
  fn v8_value_to_number(this: &Value) -> Number;
  fn v8_value_to_integer(this: &Value) -> Integer;
  fn v8_value_as_int32(this: &Value) -> i32;
  fn v8_value_as_int64(this: &Value) -> i64;
  fn v8_value_as_uint32(this: &Value) -> u32;

  fn v8_string_new_from_utf8(data: *const libc::c_char) -> String;
  fn v8_string_empty(this: &String) -> String;
  fn v8_string_as_string(this: &String) -> *const libc::c_char;

  fn v8_object_new() -> Object;
  fn v8_object_get(this: &Object, key: &Value) -> Value;
  fn v8_object_set(this: &Object, key: &Value, val: &Value) -> bool;

  fn v8_function_call(this: &Function, global: &Value, argv: &[Value]) -> Value;
  fn v8_function_callback_info_at(this: &FunctionCallbackInfo, index: i32) -> Value;

  // fn v8_function_tmpl_new() -> FunctionTemplate;
  fn v8_function_tmpl_new_with_callback(callback: &FunctionCallback) -> FunctionTemplate;
  fn v8_function_tmpl_get_function(this: &FunctionTemplate) -> Function;
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
    impl $ty {
      #[inline(always)]
      pub fn IsString(&self) -> bool {
        unsafe { v8_value_is_string(self.as_val()) }
      }
      #[inline(always)]
      pub fn ToString(&self) -> String {
        unsafe { v8_value_to_string(self.as_val()) }
      }
      #[inline(always)]
      pub fn ToNumber(&self) -> Number {
        unsafe { v8_value_to_number(self.as_val()) }
      }
      #[inline(always)]
      pub fn ToInteger(&self) -> Integer {
        unsafe { v8_value_to_integer(self.as_val()) }
      }
      pub fn Int32Value(&self) -> i32 {
        unsafe { v8_value_as_int32(self.as_val()) }
      }
      pub fn IntegerValue(&self) -> i64 {
        unsafe { v8_value_as_int64(self.as_val()) }
      }
      pub fn Uint32Value(&self) -> u32 {
        unsafe { v8_value_as_uint32(self.as_val()) }
      }
    }
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
    let c_pdata = CString::new(data).unwrap();
    unsafe { v8_string_new_from_utf8(c_pdata.as_ptr()) }
  }
  pub fn Empty(&self) -> String {
    unsafe { v8_string_empty(self) }
  }
  pub fn as_string(&self) -> string::String {
    unsafe { 
      let mut v: Vec<u8> = Vec::new();
      for i in CStr::from_ptr(v8_string_as_string(self)).to_bytes() {
        v.push(*i);
      }
      string::String::from_utf8(v).unwrap()
    }
  }
}

#[repr(C)]
pub struct Number(*mut *mut Number);
value_method!(Number);

#[repr(C)]
pub struct Integer(*mut *mut Integer);
value_method!(Integer);

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
pub struct Function(*mut *mut Function);
value_method!(Function);

impl Function {
  pub fn Call<T: ValueT>(&self, recv: T, argv: &[Value]) -> Value {
    unsafe { v8_function_call(self, recv.as_val(), argv) }
  }
}

#[repr(C)]
pub type FunctionCallback = extern fn(FunctionCallbackInfo);

#[repr(C)]
pub struct FunctionCallbackInfo(*mut *mut FunctionCallbackInfo);
impl FunctionCallbackInfo {
  pub fn At(&self, index: i32) -> Value {
    unsafe { v8_function_callback_info_at(self, index) }
  }
}

#[repr(C)]
pub struct FunctionTemplate(*mut *mut FunctionTemplate);
impl FunctionTemplate {
  pub fn New(callback: FunctionCallback) -> FunctionTemplate {
    unsafe { v8_function_tmpl_new_with_callback(&callback) }
  }
  pub fn GetFunction(&self) -> Function {
    unsafe { v8_function_tmpl_get_function(self) }
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
