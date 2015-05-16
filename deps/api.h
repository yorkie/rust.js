#ifndef RUSTJS_DEPS_C_API_H_
#define RUSTJS_DEPS_C_API_H_

#ifdef __cplusplus
extern "C" {
#endif

bool v8_initialize();
bool v8_dispose();

#ifdef __cplusplus
}
#endif

#endif
