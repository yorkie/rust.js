
#include "v8.h"
#include "api.h"

extern "C" {

bool v8_initialize() {
  return V8::Initialize();
}

}
