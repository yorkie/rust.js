extern crate rustc_serialize;

use util::v8;
use self::rustc_serialize::hex::{FromHex, ToHex};
use self::rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};

enum Encoding {
  ASCII,
  Binary,
  UTF8,
  UCS2,
  Base64,
  Hex
}

struct StringBytes {
  source: String
}

impl StringBytes {
  pub fn new(src: &'static str) -> StringBytes {
    let mut bytes_string = String::new();
    bytes_string.push_str(src);
    StringBytes {
      source: bytes_string
    }
  }
  pub fn decode(&self, encoding: Encoding) -> Vec<u8> {
    match encoding {
      Encoding::Base64 => self.source.as_bytes().from_base64().unwrap(),
      Encoding::Hex => self.source.from_hex().unwrap(),
      _ => Vec::new()
    }
  }
  pub fn encode(&self, encoding: Encoding) -> String {
    match encoding {
      Encoding::Base64 => self.source.as_bytes().to_base64(STANDARD),
      Encoding::Hex => self.source.as_bytes().to_hex(),
      _ => String::new()
    }
  }
  pub fn byte_length(&self) -> usize {
    self.source.as_bytes().len()
  }

  extern fn New(arguments: v8::FunctionCallbackInfo) {
    arguments.GetReturnValue().SetWithBool(true);
  }

}

#[test]
fn it_encodes_to_base64() {
  let bytes = StringBytes::new("test string");
  assert_eq!(bytes.encode(Encoding::Base64), "dGVzdCBzdHJpbmc=");
}

#[test]
fn it_encodes_to_hex() {
  let bytes = StringBytes::new("test string");
  assert_eq!(bytes.encode(Encoding::Hex), "7465737420737472696e67");
}

pub fn Init() -> v8::Object {
  let exports = v8::Object::New();
  exports.Set(v8::String::NewFromUtf8("StringBytes"), 
    v8::FunctionTemplate::New(StringBytes::New).GetFunction());
  return exports;
}
