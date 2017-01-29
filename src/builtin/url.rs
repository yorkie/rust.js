
use util::v8;
use url::Url;

extern fn parse(info: v8::FunctionCallbackInfo) {
  let arg = info.At(0).ToString().as_string();
  let url = Url::parse(&*arg).unwrap();
  let obj = v8::Object::New();

  // protocol
  obj.Set(v8::String::NewFromUtf8("protocol"), 
    v8::String::NewFromUtf8(url.scheme()));

  // auth
  obj.Set(v8::String::NewFromUtf8("auth"),
    v8::Boolean::New(url.has_authority()));

  // username
  obj.Set(v8::String::NewFromUtf8("username"),
    v8::String::NewFromUtf8(url.username()));

  // password
  match url.password() {
    Some(password) => {
      obj.Set(v8::String::NewFromUtf8("password"), 
        v8::String::NewFromUtf8(password));
    },
    None => {}
  }

  // hostname
  obj.Set(v8::String::NewFromUtf8("hostname"),
    v8::String::NewFromUtf8(url.host_str().unwrap()));

  // port
  match url.port_or_known_default() {
    Some(port) => {
      obj.Set(v8::String::NewFromUtf8("port"),
        v8::Number::NewFromUInt16(port));
    },
    None => {}
  }

  // pathname
  obj.Set(v8::String::NewFromUtf8("pathname"),
    v8::String::NewFromUtf8(url.path()));

  // query
  match url.query() {
    Some(query) => {
      obj.Set(v8::String::NewFromUtf8("query"),
        v8::String::NewFromUtf8(query));
    },
    None => {}
  }

  // hash
  match url.fragment() {
    Some(fragment) => {
      obj.Set(v8::String::NewFromUtf8("hash"),
        v8::String::NewFromUtf8(fragment));
    },
    None => {}
  }
  info.GetReturnValue().Set(obj);
}

extern fn format(info: v8::FunctionCallbackInfo) {

}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("parse"), 
    v8::FunctionTemplate::New(parse).GetFunction());
  exports.Set(v8::String::NewFromUtf8("format"),
    v8::FunctionTemplate::New(format).GetFunction());
  return exports;
}
