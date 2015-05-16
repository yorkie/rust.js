#ifndef RUSTJS_DEPS_C_API_H_
#define RUSTJS_DEPS_C_API_H_

#include "v8.h"

using namespace v8;

#ifdef __cplusplus
extern "C" {
#endif

int32_t v8_runtime(char *data);
bool v8_free_platform();
bool v8_initialize_platform();
bool v8_initialize();
bool v8_dispose();
bool v8_set_array_buffer_allocator();

Isolate *v8_isolate_new();
void v8_isolate_dispose(Isolate *data);

bool v8_value_isArgumentsObject(void *data);
bool v8_value_isArray(void *data);

#ifdef __cplusplus
}
#endif

#endif
