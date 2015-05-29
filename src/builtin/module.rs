
use util::v8;

extern fn require(arguments: v8::FunctionCallbackInfo) {
  let path = arguments.At(0);
}

pub fn GetRequire() -> v8::Function {
  v8::FunctionTemplate::New(require).GetFunction()
}

pub fn GetModule() {

}
