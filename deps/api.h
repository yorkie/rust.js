#ifndef RUSTJS_DEPS_C_API_H_
#define RUSTJS_DEPS_C_API_H_

#include "v8.h"

using namespace v8;

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*rust_callback)();

// bool v8_free_platform();
// bool v8_initialize_platform();
// bool v8_initialize();
// bool v8_dispose();
// bool v8_set_array_buffer_allocator();

// bool v8_locker_is_locked();
// bool v8_locker_is_active();
// void v8_locker(rust_callback callback);
// void v8_handle_scope(rust_callback callback);

// void v8_isolate_new();
// void v8_isolate_dispose();
// void v8_isolate_enter();
// void v8_isolate_exit();

// void v8_context_new();
// void v8_context_enter();
// void v8_context_exit();
// void v8_context_global();
// void v8_context_scope(rust_callback callback);

// Local<Script> v8_script_compile(char *data);
// void v8_script_run(Script **script);

// Local<String> v8_string_new_from_utf8(char *data);
// Local<String> v8_string_empty(String **str);

// Local<Object> v8_object_new();

// Local<FunctionTemplate> v8_function_tmpl_new();
// void v8_function_tmpl_set_class_name(FunctionTemplate **ft, char *name);
// Local<Object> v8_function_tmpl_new_instance(FunctionTemplate **ft);

#ifdef __cplusplus
}
#endif

#endif
