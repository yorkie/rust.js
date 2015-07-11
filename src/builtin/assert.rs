
use util::v8;

extern fn assert(arguments: v8::FunctionCallbackInfo) {
  let expr = arguments.At(0).ToBoolean().Value();
  let message = arguments.At(1).ToString().as_string();
  assert!(expr, message);
}

extern fn assert_eq(arguments: v8::FunctionCallbackInfo) {
  let expect = arguments.At(0).ToString().as_string();
  let actual = arguments.At(1).ToString().as_string();
  assert_eq!(expect, actual);
}

extern fn assert_neq(arguments: v8::FunctionCallbackInfo) {
  let expect = arguments.At(0).ToString().as_string();
  let actual = arguments.At(1).ToString().as_string();
  assert!(expect != actual);
}

pub fn Init() -> v8::Object {
  let exports = v8::FunctionTemplate::New(assert).GetFunction().ToObject();
  exports.Set(v8::String::NewFromUtf8("ok"), 
    v8::FunctionTemplate::New(assert).GetFunction());
  exports.Set(v8::String::NewFromUtf8("equal"),
    v8::FunctionTemplate::New(assert_eq).GetFunction());
  exports.Set(v8::String::NewFromUtf8("notEqual"),
    v8::FunctionTemplate::New(assert_neq).GetFunction());
  return exports;
}
