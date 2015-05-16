#ifndef RUSTJS_DEPS_C_API_H_
#define RUSTJS_DEPS_C_API_H_

#include "v8.h"

using namespace v8;

#ifdef __cplusplus
extern "C" {
#endif

int v8_runtime(char *data);
bool v8_initialize();
bool v8_dispose();
bool v8_value_isArgumentsObject(void *data);
bool v8_value_isArray(void *data);

#ifdef __cplusplus
}
#endif

#endif
