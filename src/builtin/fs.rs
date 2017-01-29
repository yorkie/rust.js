use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;
use std::str;
use util::v8;

extern fn rename(info: v8::FunctionCallbackInfo) {
  let oldpath = info.At(0).ToString().as_string();
  let newpath = info.At(1).ToString().as_string();
  v8_try!(fs::rename(oldpath, newpath), info);
  info.GetReturnValue().SetWithBool(true);
}

extern fn chown(info: v8::FunctionCallbackInfo) {
  info.GetReturnValue().SetWithBool(true);
}

extern fn rmdir(info: v8::FunctionCallbackInfo) {
  let path = info.At(0).ToString().as_string();
  v8_try!(fs::remove_dir(path), info);
  info.GetReturnValue().Set(info.At(0).ToString());
}

extern fn mkdir(info: v8::FunctionCallbackInfo) {
  let path = info.At(0).ToString().as_string();
  v8_try!(fs::create_dir(path), info);
  info.GetReturnValue().Set(info.At(0).ToString());
}

extern fn stat(info: v8::FunctionCallbackInfo) {
  let path = info.At(0).ToString().as_string();
  let meta = v8_try!(fs::metadata(path), info);
  let obj = v8::Object::New();
  obj.Set(v8::String::NewFromUtf8("dev"), v8::Number::NewFromUInt64(meta.dev()));
  obj.Set(v8::String::NewFromUtf8("ino"), v8::Number::NewFromUInt64(meta.ino()));
  obj.Set(v8::String::NewFromUtf8("mode"), v8::Number::NewFromUInt32(meta.mode()));
  obj.Set(v8::String::NewFromUtf8("nlink"), v8::Number::NewFromUInt64(meta.nlink()));
  obj.Set(v8::String::NewFromUtf8("uid"), v8::Number::NewFromUInt32(meta.uid()));
  obj.Set(v8::String::NewFromUtf8("gid"), v8::Number::NewFromUInt32(meta.gid()));
  obj.Set(v8::String::NewFromUtf8("rdev"), v8::Number::NewFromUInt64(meta.rdev()));
  obj.Set(v8::String::NewFromUtf8("size"), v8::Number::NewFromUInt64(meta.size()));
  obj.Set(v8::String::NewFromUtf8("blksize"), v8::Number::NewFromUInt64(meta.blksize()));
  obj.Set(v8::String::NewFromUtf8("blocks"), v8::Number::NewFromUInt64(meta.blocks()));
  obj.Set(v8::String::NewFromUtf8("atime"), v8::Number::NewFromInt64(meta.atime()));
  obj.Set(v8::String::NewFromUtf8("mtime"), v8::Number::NewFromInt64(meta.mtime()));
  obj.Set(v8::String::NewFromUtf8("ctime"), v8::Number::NewFromInt64(meta.ctime()));
  info.GetReturnValue().Set(obj);
}

extern fn readdir(info: v8::FunctionCallbackInfo) {
  let path = info.At(0).ToString().as_string();
  let dir = v8_try!(fs::read_dir(path), info);
  let ret = v8::Array::New();
  for entry in dir {
    let entry = v8_try!(entry, info);
    let path = entry.path();
    ret.push(v8::String::NewFromUtf8(path.to_str().unwrap()));
  }
  info.GetReturnValue().Set(ret);
}

extern fn readFile(info: v8::FunctionCallbackInfo) {
  let path = info.At(0).ToString().as_string();
  let mut f = v8_try!(File::open(path), info);
  let mut s = String::new();
  v8_try!(f.read_to_string(&mut s), info);

  let val = v8_try!(str::from_utf8(s.as_bytes()), info);
  let ret = v8::String::NewFromUtf8(val);
  info.GetReturnValue().Set(ret);
}

extern fn writeFile(info: v8::FunctionCallbackInfo) {
  // let path = info.At(0).ToString().as_string();
  // let buf = info.At(1).ToString();

  info.GetReturnValue().SetWithBool(true);
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
