
use util::v8;
use tokio_proto::TcpClient;

struct Socket;
impl Socket {
  extern fn New(info: v8::FunctionCallbackInfo) {
    if info.IsConstructCall() {
      info.GetReturnValue().Set(info.This());
    }
  }
  extern fn Connect(info: v8::FunctionCallbackInfo) {
    // TODO(Yorkie)
  }
  extern fn Disconnect(info: v8::FunctionCallbackInfo) {
    // TODO(Yorkie)
  }
  extern fn Init() -> v8::Function {
    let tpl = v8::FunctionTemplate::New(Socket::New);
    tpl.SetClassName("Socket");
    tpl.SetInternalFieldCount(1);
    tpl.SetPropertyMethod("connect", Socket::Connect);
    tpl.SetPropertyMethod("disconnect", Socket::Disconnect);
    tpl.GetFunction()
  }
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("Socket"), Socket::Init());
  exports
}
