
use util::v8;

extern fn assert(arguments: v8::FunctionCallbackInfo) {
  let expr = arguments.At(0).ToBoolean().Value();
  let message = arguments.At(1).ToString().as_string();
  assert!(expr, message);
}

extern fn assert_equal(arguments: v8::FunctionCallbackInfo) {
  let expect = arguments.At(0).ToString().as_string();
  let actual = arguments.At(1).ToString().as_string();
  assert_eq!(expect, actual);
}

pub fn Init() -> v8::Object {
  let exports = v8::FunctionTemplate::New(assert).GetFunction().ToObject();
  exports.Set(v8::String::NewFromUtf8("ok"), 
    v8::FunctionTemplate::New(assert).GetFunction());
  exports.Set(v8::String::NewFromUtf8("equal"),
    v8::FunctionTemplate::New(assert_equal).GetFunction());
  return exports;
}
