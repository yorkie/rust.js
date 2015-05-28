
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

// The global isolate
Isolate *isolate;
// The global try catch
TryCatch try_catch;
// The global context
Local<Context> context;

/**
 * v8 methods
 * @class v8
 */

/**
 * free the platform variable
 * @method v8_free_platform()
 * @return {bool} always return true
 */
bool v8_free_platform() {
  delete default_platform;
  default_platform = nullptr;
  return true;
}

/**
 * initialize the platform
 * @method v8_initialize_platform
 * @return {bool} always return true
 */
bool v8_initialize_platform() {
  if (default_platform == nullptr) {
    default_platform = platform::CreateDefaultPlatform();
    V8::InitializePlatform(default_platform);
  }
  return true;
}

/**
 * initilize the v8 instance
 * @method v8_initialize
 * @return {bool} if there is some error during the process
 */
bool v8_initialize() {
  return V8::Initialize();
}

/**
 * dispose the v8
 * @method v8_dispose
 * @return {bool} if there is some error during the process
 */
bool v8_dispose() {
  return V8::Dispose();
}

/**
 * set the global array buffer
 * @method v8_set_array_buffer_allocator
 * @return {bool} always be true
 */
bool v8_set_array_buffer_allocator() {
  V8::SetArrayBufferAllocator(&ArrayBufferAllocator::the_singleton);
  return true;
}

/**
 * check if the global isolate is locked
 * @method v8_locker_is_locked
 * @return {bool} the result, true or false
 */
bool v8_locker_is_locked() {
  return Locker::IsLocked(isolate);
}

/**
 * check if the global isolate is active
 * @method v8_locker_is_active
 * @return {bool} the result, true or false
 */
bool v8_locker_is_active() {
  return Locker::IsActive();
}

/**
 * create new locker with a locked callback
 * @method v8_locker
 * @param {rust_callback} callback - The rust closure function
 * @return {void} doesn't return anything
 */
void v8_locker(rust_callback callback) {
  Locker locker(isolate);
  callback();
}

/**
 * create new handle scope with a scoped callback
 * @method v8_handle_scope
 * @param {rust_callback} callback - The rust closure function
 * @return {void} doesn't return anything
 */
void v8_handle_scope(rust_callback callback) {
  HandleScope handle_scope(isolate);
  callback();
}

/**
 * create new isolate and assign to global isolate variable
 * @method v8_isolate_new
 * @return {void} doesn't return anything
 */
void v8_isolate_new() {
  isolate = Isolate::New();
}

/**
 * dispose the global isolate
 * @method v8_isolate_dispose
 * @return {void} doesn't return anything
 */
void v8_isolate_dispose() {
  isolate->Dispose();
  isolate = nullptr;
}

/**
 * call isolate->Enter() at rust level
 * @method v8_isolate_enter
 * @return {void} doesn't return anything
 */
void v8_isolate_enter() {
  isolate->Enter();
}

/**
 * call isolate->Exit() at rust level
 * @method v8_isolate_exit
 * @return {void} doesn't return anything
 */
void v8_isolate_exit() {
  isolate->Exit();
}

/**
 * create new context and assign to global context variable
 * @method v8_context_new
 * @return {void} doesn't return anything
 */
void v8_context_new() {
  context = Context::New(isolate);
}

/**
 * call context->Enter() at rust level
 * @method v8_context_enter
 * @return {void} doesn't return anything
 */
void v8_context_enter() {
  context->Enter();
}

/**
 * call context->Exit() at rust level
 * @method v8_context_exit
 * @return {void} doesn't return anything
 */
void v8_context_exit() {
  context->Exit();
}

/**
 * get global variable namely context->Global()
 * @method v8_context_global
 * @return {Value} The global value
 */
Local<Value> v8_context_global() {
  return context->Global();
}

/**
 * create new context scope with a scoped callback
 * @method v8_context_scope
 * @param {rust_callback} callback - The rust closure function
 * @return {void} doesn't return anything
 */
void v8_context_scope(rust_callback callback) {
  Context::Scope context_scope(context);
  callback();
}

/**
 * The script class
 * @class Script
 */

/**
 * compile a script
 * @method v8_script_compile
 * @param {char *} data - The source of script
 * @return {Script} the compiled script object
 */
Local<Script> v8_script_compile(char *data) {
  Local<String> source = String::NewFromUtf8(isolate, data);
  Local<Script> script = Script::Compile(source);
  return script;
}

/**
 * run the script
 * @method v8_script_run
 * @param {Script} this
 * @return {Value} the result
 */
Local<Value> v8_script_run(Script **script) {
  return (*script)->Run();
}

/**
 * The value class
 * @class Value
 */

/**
 * check the value if it's a string
 * @method v8_value_is_string
 * @param {Value} this
 * @return {bool} the result
 */
bool v8_value_is_string(Value **val) {
  return (*val)->IsString();
}

/**
 * call val->ToString()
 * @method v8_value_to_string
 * @param {Value} this
 * @return {String} the result
 */
Local<String> v8_value_to_string(Value **val) {
  return (*val)->ToString();
}

/**
 * The String class
 * @class String
 */

/**
 * create new v8::String from utf8 string
 * @method v8_string_new_from_utf8
 * @param {char*} data
 * @return {String} the v8::String value
 */
Local<String> v8_string_new_from_utf8(char *data) {
  return String::NewFromUtf8(isolate, data);
}

/**
 * empty the current string
 * @method v8_string_empty
 * @param {String} this
 * @return {String}
 */
Local<String> v8_string_empty(String **str) {
  return (*str)->Empty(isolate);
}

/**
 * The Object class
 * @class Object
 */

/**
 * create new object in current isolate
 * @method v8_object_new
 * @return {Object} the returned object
 */
Local<Object> v8_object_new() {
  return Object::New(isolate);
}

/**
 * get the value by key from an object
 * @method v8_object_get
 * @param {Object} this
 * @param {Value} key
 * @return {Value} The value
 */
Local<Value> v8_object_get(Object **object, Local<Value> *key) {
  return (*object)->Get(*key);
}

/**
 * set the value of key from an object
 * @method v8_object_set
 * @param {Object} this
 * @param {Value} key
 * @param {Value} val
 * @return {bool}
 */
bool v8_object_set(Object **object, Local<Value> *key, Local<Value> *val) {
  return (*object)->Set(*key, *val);
}

/**
 * The Function class
 * @class Function
 */

/**
 * call the function with global and args
 * @method v8_function_call
 * @param {Function} this
 * @param {Value} global - The global variable
 * @param {Value[]} argv - The arguments
 * @return {Value} the result
 */
Local<Value> v8_function_call(Function **func, Local<Value> global, Local<Value> **argv) {
  return (*func)->Call(global, 2, *argv);
}

/**
 * The FunctionCallbackInfo class
 * @class FunctionCallbackInfo
 */

/**
 * get value from function arguments by index
 * @method v8_function_callback_info_at
 * @param {FunctionCallbackInfo} callbackInfo
 * @param {int} index
 * @return {Value} the result
 */
Local<Value> v8_function_callback_info_at(FunctionCallbackInfo<Value> **callbackInfo, int index) {
  return (**callbackInfo)[index];
}

/**
 * The FunctionTemplate class
 * @class FunctionTemplate
 */

/**
 * create new FunctionTemplate instance
 * @method v8_function_tmpl_new
 * @constructor
 * @return {FunctionTemplate} new instance
 */
Local<FunctionTemplate> v8_function_tmpl_new() {
  return FunctionTemplate::New(isolate);
}

/**
 * create new FunctionTemplate instance with callback
 * @method v8_function_tmpl_new
 * @constructor
 * @param {FunctionCallback} callback
 * @return {FunctionTemplate} new instance
 */
Local<FunctionTemplate> v8_function_tmpl_new_with_callback(FunctionCallback *callback) {
  return FunctionTemplate::New(isolate, *callback);
}

/**
 * get Function object from FunctionTemplate
 * @method v8_function_tmpl_get_function
 * @param {FunctionTemplate} this
 * @return {Function} the result
 */
Local<Function> v8_function_tmpl_get_function(FunctionTemplate **ft) {
  return (*ft)->GetFunction();
}

/**
 * set class name of FunctionTemplate object
 * @method v8_function_tmpl_set_class_name
 * @param {FunctionTemplate} this
 * @param {char*} the class name
 * @return {void}
 */
void v8_function_tmpl_set_class_name(FunctionTemplate **ft, char *name) {
  (*ft)->SetClassName(String::NewFromUtf8(isolate, name));
}

/**
 * get a new instance(object) from FunctionTemplate
 * @method v8_function_tmpl_new_instance
 * @param {FunctionTemplate} this
 * @return {Object} the returned object
 */
Local<Object> v8_function_tmpl_new_instance(FunctionTemplate **ft) {
  return (*ft)->GetFunction()->NewInstance();
}

}