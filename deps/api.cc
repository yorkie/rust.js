
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string>

#include "v8.h"
#include "api.h"
#include "libplatform/libplatform.h"

using namespace v8;

class ArrayBufferAllocator : public ArrayBuffer::Allocator {
 public:
  static ArrayBufferAllocator the_singleton;
  virtual void* Allocate(size_t length);
  virtual void* AllocateUninitialized(size_t length);
  virtual void Free(void* data, size_t);
};

ArrayBufferAllocator ArrayBufferAllocator::the_singleton;

void *ArrayBufferAllocator::Allocate(size_t length) {
  void* data = AllocateUninitialized(length);
  return data == NULL ? data : memset(data, 0, length);
}

void* ArrayBufferAllocator::AllocateUninitialized(size_t length) {
  return malloc(length);
}

void ArrayBufferAllocator::Free(void *data, size_t) {
  free(data);
}

extern "C" {

static Platform* default_platform;
static ArrayBufferAllocator array_buffer_allocator;

int v8_runtime(char *data) {
  default_platform = platform::CreateDefaultPlatform();
  V8::InitializePlatform(default_platform);
  V8::Initialize();
  V8::SetArrayBufferAllocator(&ArrayBufferAllocator::the_singleton);

  int code = 1;
  Isolate* isolate = Isolate::New();
  {
    Locker locker(isolate);
    Isolate::Scope isolate_scope(isolate);
    HandleScope handle_scope(isolate);
    Local<Context> context = Context::New(isolate);
    Context::Scope context_scope(context);

    TryCatch try_catch;
    Local<String> name = String::NewFromUtf8(isolate, "main");
    Local<String> source = String::NewFromUtf8(isolate, data);

    ScriptOrigin origin(name);
    Local<Script> script = Script::Compile(source, &origin);

    if (script.IsEmpty()) {
      printf("empty script\n");
      assert(try_catch.HasCaught());
    } else {
      Handle<Value> result = script->Run();
      if (result.IsEmpty()) {
        assert(try_catch.HasCaught());
        code = 2;
      }
      printf("process exited normally\n");
    }
  }
  isolate->Dispose();
  isolate = nullptr;

  V8::Dispose();
  delete default_platform;
  default_platform = nullptr;
  return code;
}

bool v8_initialize() {
  return V8::Initialize();
}

bool v8_dispose() {
  return V8::Dispose();
}

bool v8_value_isArgumentsObject(void *data) {
  Value *instance = static_cast<Value *>(data);
  return instance->IsArgumentsObject();
}

bool v8_value_isArray(void *data) {
  Value *instance = static_cast<Value *>(data);
  return instance->IsArray();
}

}