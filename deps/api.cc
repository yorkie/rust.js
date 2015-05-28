
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

Isolate *isolate;
TryCatch try_catch;
Local<Context> context;

bool v8_free_platform() {
  delete default_platform;
  default_platform = nullptr;
  return true;
}

bool v8_initialize_platform() {
  if (default_platform == nullptr) {
    default_platform = platform::CreateDefaultPlatform();
    V8::InitializePlatform(default_platform);
  }
  return true;
}

bool v8_initialize() {
  return V8::Initialize();
}

bool v8_dispose() {
  return V8::Dispose();
}

bool v8_set_array_buffer_allocator() {
  V8::SetArrayBufferAllocator(&ArrayBufferAllocator::the_singleton);
  return true;
}

bool v8_locker_is_locked() {
  return Locker::IsLocked(isolate);
}

bool v8_locker_is_active() {
  return Locker::IsActive();
}

void v8_locker(rust_callback callback) {
  Locker locker(isolate);
  callback();
}

void v8_handle_scope(rust_callback callback) {
  HandleScope handle_scope(isolate);
  callback();
}

void v8_isolate_new() {
  isolate = Isolate::New();
}

void v8_isolate_dispose() {
  isolate->Dispose();
  isolate = nullptr;
}

void v8_isolate_enter() {
  isolate->Enter();
}

void v8_isolate_exit() {
  isolate->Exit();
}

void v8_context_new() {
  context = Context::New(isolate);
}

void v8_context_enter() {
  context->Enter();
}

void v8_context_exit() {
  context->Exit();
}

void v8_context_global() {
  context->Global();
}

void v8_context_scope(rust_callback callback) {
  Context::Scope context_scope(context);
  callback();
}

bool v8_value_isArgumentsObject(void *data) {
  Value *instance = static_cast<Value *>(data);
  return instance->IsArgumentsObject();
}

bool v8_value_isArray(void *data) {
  Value *instance = static_cast<Value *>(data);
  return instance->IsArray();
}

Local<Script> v8_script_compile(char *data) {
  Local<String> source = String::NewFromUtf8(isolate, data);
  Local<Script> script = Script::Compile(source);
  return script;
}

void v8_script_run(Script **script) {
  Local<Value> result = (*script)->Run();
}

Local<String> v8_string_new_from_utf8(char *data) {
  return String::NewFromUtf8(isolate, data);
}

Local<String> v8_string_empty(String **str) {
  return (*str)->Empty(isolate);
}

Local<Object> v8_object_new() {
  return Object::New(isolate);
}

Local<Value> v8_object_get(Object **object, Local<Value> key) {
  return (*object)->Get(key);
}

bool v8_object_set(Object **object, Local<Value> *key, Local<Value> *val) {
  return (*object)->Set(*key, *val);
}

Local<FunctionTemplate> v8_function_tmpl_new() {
  return FunctionTemplate::New(isolate);
}

void v8_function_tmpl_set_class_name(FunctionTemplate **ft, char *name) {
  (*ft)->SetClassName(String::NewFromUtf8(isolate, name));
}

Handle<Object> v8_function_tmpl_new_instance(FunctionTemplate **ft) {
  return (*ft)->GetFunction()->NewInstance();
}

}