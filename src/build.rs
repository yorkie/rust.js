
// ARCH := $(word 1, $(subst -, ,$(TARGET)))

// V8_FLAGS := OUTDIR="$(OUT_DIR)" library=shared i18nsupport=off

// ifeq (true,$(DEBUG))
// V8_FLAGS := $(V8_FLAGS) extrachecks=on
// V8_MODE := debug
// else
// V8_MODE := release
// endif

// ifeq (i686,$(ARCH))
// V8_ARCH := ia32
// endif

// ifeq (x86_64,$(ARCH))
// V8_ARCH := x64
// endif

// V8_TARGET := $(V8_ARCH).$(V8_MODE)

// .PHONY: all
// all: $(OUT_DIR)/libv8.so

// $(OUT_DIR)/libv8.so:
//   $(MAKE) -f makefile.v8 -j $(NUM_JOBS) $(V8_TARGET) $(V8_FLAGS)
//   ln -fs $(OUT_DIR)/$(V8_TARGET)/lib.target/libv8.so $@

use std::process::Command;

fn main() {
  let output = Command::new("make")
    .args(&["-f", "./deps/v8/Makefile"])
    .output()
    .unwrap_or_else(|e| {
      panic!("failed to execute process: {}", e) 
    });
  println!("cargo:{:?}", output.stdout);

}