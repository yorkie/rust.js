
use std::collections::HashMap;
use util::v8;

pub struct ModulesHeap {
  modules: HashMap<&'static str, v8::Object>
}

impl ModulesHeap {
  pub fn New() -> ModulesHeap {
    ModulesHeap {
      modules: HashMap::new()
    }
  }
  pub fn binding(&mut self, name: &'static str, val: v8::Object) {
    self.modules.insert(name, val);
  }
  pub fn get(&self, name: &'static str) -> &v8::Object {
    self.modules.get(&name).unwrap()
  }
}