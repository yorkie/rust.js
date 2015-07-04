
use util::v8;
use builtin::module;

pub fn Init() -> v8::Object {
  module::LoadBuiltinScript("events")
}
