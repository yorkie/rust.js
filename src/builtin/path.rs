
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use util::v8;

extern fn normalize(arguments: v8::FunctionCallbackInfo) {
  let p = arguments.At(0).ToString().as_string();
  let path = Path::new(&*p).to_str();
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(path.unwrap()));
}

extern fn join(arguments: v8::FunctionCallbackInfo) {
  // TODO
  arguments.GetReturnValue().SetWithBool(true);
}

extern fn resolve(arguments: v8::FunctionCallbackInfo) {
  // TODO
  arguments.GetReturnValue().SetWithBool(true);
}

extern fn isAbsolute(arguments: v8::FunctionCallbackInfo) {
  let p = arguments.At(0).ToString().as_string();
  arguments.GetReturnValue().SetWithBool(Path::new(&*p).is_absolute());
}

extern fn relative(arguments: v8::FunctionCallbackInfo) {
  // TODO
  arguments.GetReturnValue().SetWithBool(true);
}

extern fn dirname(arguments: v8::FunctionCallbackInfo) {
  let p = arguments.At(0).ToString().as_string();
  let dirpath = Path::new(&*p).parent().unwrap().to_str();
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(dirpath.unwrap()));
}

extern fn basename(arguments: v8::FunctionCallbackInfo) {
  let p = arguments.At(0).ToString().as_string();
  let filename = Path::new(&*p).file_name().unwrap().to_str();
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(filename.unwrap()));
}

extern fn extname(arguments: v8::FunctionCallbackInfo) {
  let p = arguments.At(0).ToString().as_string();
  let extname = Path::new(&*p).extension().unwrap().to_str();
  arguments.GetReturnValue().Set(v8::String::NewFromUtf8(extname.unwrap()));
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("normalize"), v8::FunctionTemplate::New(normalize).GetFunction());
  exports.Set(v8::String::NewFromUtf8("join"), v8::FunctionTemplate::New(join).GetFunction());
  exports.Set(v8::String::NewFromUtf8("resolve"), v8::FunctionTemplate::New(resolve).GetFunction());
  exports.Set(v8::String::NewFromUtf8("isAbsolute"), v8::FunctionTemplate::New(isAbsolute).GetFunction());
  exports.Set(v8::String::NewFromUtf8("relative"), v8::FunctionTemplate::New(relative).GetFunction());
  exports.Set(v8::String::NewFromUtf8("dirname"), v8::FunctionTemplate::New(dirname).GetFunction());
  exports.Set(v8::String::NewFromUtf8("basename"), v8::FunctionTemplate::New(basename).GetFunction());
  exports.Set(v8::String::NewFromUtf8("extname"), v8::FunctionTemplate::New(extname).GetFunction());
  exports.Set(v8::String::NewFromUtf8("sep"), v8::String::NewFromUtf8(&*MAIN_SEPARATOR.to_string()));
  return exports;
}
