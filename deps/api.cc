
#include "v8.h"
#include "api.h"

extern "C" {

bool v8_initialize() {
  printf("calling v8_initialize function\n");
  return v8::V8::Initialize();
}

bool v8_dispose() {
  return v8::V8::Dispose();
}

int main(int argc, char const *argv[]) {
  v8_initialize();
  v8_dispose();
  return 0;
}

}
