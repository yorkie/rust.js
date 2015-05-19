#ifndef RUSTJS_DEPS_C_API_H_
#define RUSTJS_DEPS_C_API_H_

#include "v8.h"

using namespace v8;

#ifdef __cplusplus
extern "C" {
#endif

#define FIXED_ONE_BYTE_STRING(isolate, string)\
  (node::OneByteString((isolate), (string), sizeof(string) - 1))

int32_t v8_runtime(char *data);
bool v8_free_platform();
bool v8_initialize_platform();
bool v8_initialize();
bool v8_dispose();
bool v8_set_array_buffer_allocator();

void v8_isolate_new();
void v8_isolate_dispose();
void v8_context_new();

bool v8_value_isArgumentsObject(void *data);
bool v8_value_isArray(void *data);

bool v8_locker_is_locked(Isolate *isolate);
bool v8_locker_is_active();
void v8_locker_initialize(Locker *locker, Isolate *isolate);

Local<Script> v8_script_compile(Isolate *isolate, char *data);

#ifdef __cplusplus
}
#endif

#endif
