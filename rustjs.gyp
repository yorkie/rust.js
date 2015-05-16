{
  'targets': [
    {
      'target_name': 'rustjs_deps',
      'type': 'shared_library',
      'dependencies': [
        'deps/v8/tools/gyp/v8.gyp:v8',
        'deps/v8/tools/gyp/v8.gyp:v8_libplatform',
      ],
      'include_dirs': [
        'deps',
        'deps/v8/include',
      ],
      'sources': [
        'deps/api.h',
        'deps/api.cc',
        'deps/v8/include/v8.h',
        'deps/v8/include/v8-debug.h',
        'common.gypi',
      ],
      'libraries': [
        '-L/Users/yorkie/workspace/rustjs/out/Release',
        '-lv8_base',
        '-lv8_libbase',
        '-lv8_libplatform',
        '-lv8_nosnapshot',
      ],
    }
  ]
}
