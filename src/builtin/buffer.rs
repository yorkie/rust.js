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
  pub fn New(src: String) -> StringBytes {
    StringBytes {
      source: src
    }
  }
  pub fn decode(&self, encoding: Encoding) -> Vec<u8> {
    match encoding {
      BASE64 => self.source.as_bytes().from_base64().unwrap()
      // HEX => self.source.as_bytes().from_hex().unwrap(),
    }
  }
  pub fn encode(&self, encoding: Encoding) {
    // TODO
  }
}

pub fn Init() -> v8::Object {
  v8::Object::New()
}
