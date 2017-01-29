
use util::v8;
use openssl;

pub fn Init() -> v8::Object {
  openssl::init();
  let exports = v8::Object::New();

  exports.Set(v8::String::NewFromUtf8("version"), 
    v8::String::NewFromUtf8(openssl::version::version()));
  exports.Set(v8::String::NewFromUtf8("platform"),
    v8::String::NewFromUtf8(openssl::version::platform()));
  exports.Set(v8::String::NewFromUtf8("number"),
    v8::Number::NewFromInt64(openssl::version::number()));
  exports.Set(v8::String::NewFromUtf8("dir"),
    v8::String::NewFromUtf8(openssl::version::dir()));

  exports
}
