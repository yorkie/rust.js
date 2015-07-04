
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use util::v8;

extern fn rename(arguments: v8::FunctionCallbackInfo) {
  let oldpath = arguments.At(0).ToString().as_string();
  let newpath = arguments.At(1).ToString().as_string();
  let retval = arguments.GetReturnValue();
  match fs::rename(oldpath, newpath) {
    Ok(()) => retval.SetWithBool(true),
    Err(e) => retval.SetWithBool(false)
  }
}

extern fn chown(arguments: v8::FunctionCallbackInfo) {
  // TODO
}

extern fn rmdir(arguments: v8::FunctionCallbackInfo) {
  let path = arguments.At(0).ToString().as_string();
  let retval = arguments.GetReturnValue();
  match fs::remove_dir(path) {
    Ok(()) => retval.Set(arguments.At(0).ToString()),
    Err(e) => retval.SetWithBool(false)
  }
}

extern fn mkdir(arguments: v8::FunctionCallbackInfo) {
  let path = arguments.At(0).ToString().as_string();
  let retval = arguments.GetReturnValue();
  match fs::create_dir(path) {
    Ok(()) => retval.Set(arguments.At(0).ToString()),
    Err(e) => retval.SetWithBool(false)
  }
}

extern fn stat(arguments: v8::FunctionCallbackInfo) {
  let path = arguments.At(0).ToString().as_string();
  let retval = arguments.GetReturnValue();
  match fs::metadata(path) {
    Ok(meta) => {
      let obj = v8::Object::New();
      obj.Set(v8::String::NewFromUtf8("dev"), v8::Number::NewFromInt32(meta.dev()));
      obj.Set(v8::String::NewFromUtf8("ino"), v8::Number::NewFromUInt64(meta.ino()));
      obj.Set(v8::String::NewFromUtf8("mode"), v8::Number::NewFromUInt16(meta.mode()));
      obj.Set(v8::String::NewFromUtf8("nlink"), v8::Number::NewFromUInt16(meta.nlink()));
      obj.Set(v8::String::NewFromUtf8("uid"), v8::Number::NewFromUInt32(meta.uid()));
      obj.Set(v8::String::NewFromUtf8("gid"), v8::Number::NewFromUInt32(meta.gid()));
      obj.Set(v8::String::NewFromUtf8("rdev"), v8::Number::NewFromInt32(meta.rdev()));
      obj.Set(v8::String::NewFromUtf8("size"), v8::Number::NewFromInt64(meta.size()));
      obj.Set(v8::String::NewFromUtf8("blksize"), v8::Number::NewFromInt32(meta.blksize()));
      obj.Set(v8::String::NewFromUtf8("blocks"), v8::Number::NewFromInt64(meta.blocks()));
      obj.Set(v8::String::NewFromUtf8("atime"), v8::Number::NewFromInt64(meta.atime()));
      obj.Set(v8::String::NewFromUtf8("mtime"), v8::Number::NewFromInt64(meta.mtime()));
      obj.Set(v8::String::NewFromUtf8("ctime"), v8::Number::NewFromInt64(meta.ctime()));
      retval.Set(obj);
    },
    Err(e) => retval.SetWithBool(false)
  }
}

extern fn readdir(arguments: v8::FunctionCallbackInfo) {
  let path = arguments.At(0).ToString().as_string();
  let retval = arguments.GetReturnValue();
  match fs::read_dir(path) {
    Ok(dir) => {
      let arr = v8::Array::New();
      retval.Set(arr);
    },
    Err(e) => retval.SetWithBool(false)
  }
}

extern fn readFile(arguments: v8::FunctionCallbackInfo) {
  // TODO
}

extern fn writeFile(arguments: v8::FunctionCallbackInfo) {
  // TODO
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("rename"), v8::FunctionTemplate::New(rename).GetFunction());
  exports.Set(v8::String::NewFromUtf8("chown"), v8::FunctionTemplate::New(chown).GetFunction());
  exports.Set(v8::String::NewFromUtf8("rmdir"), v8::FunctionTemplate::New(rmdir).GetFunction());
  exports.Set(v8::String::NewFromUtf8("mkdir"), v8::FunctionTemplate::New(mkdir).GetFunction());
  exports.Set(v8::String::NewFromUtf8("stat"), v8::FunctionTemplate::New(stat).GetFunction());
  exports.Set(v8::String::NewFromUtf8("readdir"), v8::FunctionTemplate::New(readdir).GetFunction());
  exports.Set(v8::String::NewFromUtf8("readFile"), v8::FunctionTemplate::New(readFile).GetFunction());
  exports.Set(v8::String::NewFromUtf8("writeFile"), v8::FunctionTemplate::New(writeFile).GetFunction());
  return exports;
}
