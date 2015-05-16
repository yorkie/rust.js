
#include "v8.h"
#include "libplatform/libplatform.h"
#include "api.h"

using namespace v8;

extern "C" {

static Platform* default_platform;

int v8_runtime() {
  default_platform = Platform::CreateDefaultPlatform(4);
  V8::InitializePlatform(default_platform);
  V8::Initialize();

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
    Local<String> source = String::NewFromUtf8(w->isolate, "var x = 10;");

    ScriptOrigin origin(name);
    Local<Script> script = Script::Compile(source, &origin);

    if (script.IsEmpty()) {
      assert(try_catch.HasCaught());
      w->last_exception = ExceptionString(w->isolate, &try_catch);
    } else {
      Handle<Value> result = script->Run();
      if (result.IsEmpty()) {
        assert(try_catch.HasCaught());
        w->last_exception = ExceptionString(w->isolate, &try_catch);
        code = 2;
      }
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