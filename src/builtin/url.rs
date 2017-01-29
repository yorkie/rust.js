
use util::v8;
use util::v8::ValueT;
use url::Url;

extern fn parse(info: v8::FunctionCallbackInfo) {
  let arg = info.At(0).ToString().as_string();
  // FIXME(Yorkie): convert string to str, any better solution?
  let url = Url::parse(&*arg).unwrap();
  let obj = v8::Object::New();

  // protocol
  obj.Set(v8::String::NewFromUtf8("protocol"), 
    v8::String::NewFromUtf8(url.scheme()));

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
  let options = info.At(0).ToObject();
  let mut url = Url::parse("http://localhost").unwrap();

  let protocol = options.Get(v8::String::NewFromUtf8("protocol"));
  let hostname = options.Get(v8::String::NewFromUtf8("hostname"));
  let port = options.Get(v8::String::NewFromUtf8("port"));
  let username = options.Get(v8::String::NewFromUtf8("username"));
  let password = options.Get(v8::String::NewFromUtf8("password"));
  let pathname = options.Get(v8::String::NewFromUtf8("pathname"));
  let query = options.Get(v8::String::NewFromUtf8("query"));
  let hash = options.Get(v8::String::NewFromUtf8("hash"));

  if protocol.IsString() {
    let val = protocol.ToString().as_string();
    url.set_scheme(&*val);
  }

  if hostname.IsString() {
    let val = hostname.ToString().as_string();
    url.set_host(Some(&*val));
  }

  if !port.IsUndefined() {
    let val = port.Int32Value() as u16;
    url.set_port(Some(val));
  }

  if username.IsString() {
    let val = username.ToString().as_string();
    url.set_username(&*val);
  }

  if password.IsString() {
    let val = password.ToString().as_string();
    url.set_password(Some(&*val));
  }

  if pathname.IsString() {
    let val = pathname.ToString().as_string();
    url.set_path(&*val);
  }

  if query.IsString() {
    let val = query.ToString().as_string();
    url.set_query(Some(&*val));
  }

  if hash.IsString() {
    let val = hash.ToString().as_string();
    url.set_fragment(Some(&*val));
  }

  info.GetReturnValue().Set(v8::String::NewFromUtf8(url.as_str()));
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("parse"), 
    v8::FunctionTemplate::New(parse).GetFunction());
  exports.Set(v8::String::NewFromUtf8("format"),
    v8::FunctionTemplate::New(format).GetFunction());
  return exports;
}
