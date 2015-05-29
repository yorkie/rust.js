
use util::v8;
use util::module::ModulesHeap;

extern fn require(arguments: v8::FunctionCallbackInfo) {
  // TODO
}

pub fn Setup(modules: ModulesHeap) -> v8::Function {
  v8::FunctionTemplate::New(require).GetFunction()
}
