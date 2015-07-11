
use util::v8;

extern fn assert(arguments: v8::FunctionCallbackInfo) {
  let expr = arguments.At(0).ToBoolean();
}

pub fn Init() -> v8::Object {
  let exports = v8::FunctionTemplate::New(assert).GetFunction();
  return exports;
}
