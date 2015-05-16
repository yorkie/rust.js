
#include "v8.h"
#include "api.h"
#include "pthread.h"

extern "C" {

bool v8_initialize() {
  return v8::V8::Initialize();
}

bool init() {
  pthread_t thread1;
  return true;
}

int main(int argc, char const *argv[]) {
  printf("haha\n");
  return 0;
}

}
