
use util::v8;
use util::v8::Function;
use util::v8::ValueT;

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  let script = v8::Script::CompileWithFile("src/builtin/events.js");
  // let fval = script.Run();

  // if fval.IsFunction() {
  //   println!("its a function type!!!");
  // }

  // let f = Function::Cast(&fval);
  // let context = v8::Context::Global();

  // let args = &[exports.as_val()];
  // f.Call(context, args);

  exports
}
