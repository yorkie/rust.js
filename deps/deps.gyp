{
  'targets': [
    {
      'target_name': 'deps',
      'type': 'static_library',
      'dependencies': [
        'v8/tools/gyp/v8.gyp:v8',
        'v8/tools/gyp/v8.gyp:v8_libplatform'
      ],
      'include_dirs': [
        'v8' # include/v8_platform.h
      ],
      'sources': [
        # TODO
        # node.gyp is added to the project by default.
        'common.gypi',
      ]
    }
  ]
}
