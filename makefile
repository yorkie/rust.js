-include deps/config.mk

BUILDTYPE ?= Release
PYTHON ?= python
CARGO ?= cargo
DESTDIR ?=
SIGN ?=
PREFIX ?= /usr/local

# Determine EXEEXT
EXEEXT := $(shell $(PYTHON) -c \
		"import sys; print('.exe' if sys.platform == 'win32' else '')")

RUSTJS ?= ./rustjs$(EXEEXT)
RUSTJS_EXE = rustjs$(EXEEXT)
RUSTJS_G_EXE = rustjs_g$(EXEEXT)
V ?= 1

# BUILDTYPE=Debug builds both release and debug builds. If you want to compile
# just the debug build, run `make -C target.deps BUILDTYPE=Debug` instead.
ifeq ($(BUILDTYPE),Release)
all: target.deps/Makefile $(RUSTJS_EXE)
else
all: target.deps/Makefile $(RUSTJS_EXE) $(RUSTJS_G_EXE)
endif

$(RUSTJS_EXE): config.gypi target.deps/Makefile
	$(MAKE) -C target.deps BUILDTYPE=Release V=$(V)
	ln -f target.deps/Release/librustjs_deps.dylib /usr/local/lib/
	$(CARGO) build
	ln -f target/debug/rustjs /usr/local/bin/

$(RUSTJS_G_EXE): deps/config.gypi deps/target.deps/Makefile
	$(MAKE) -C target.deps BUILDTYPE=Debug V=$(V)
	ln -f target.deps/Release/librustjs_deps.dylib /usr/local/lib/
	$(CARGO) build
	ln -f target/debug/rustjs /usr/local/bin/

deps/target.deps/Makefile: deps/common.gypi deps/v8/build/toolchain.gypi deps/v8/build/features.gypi deps/v8/tools/gyp/v8.gyp deps/deps.gyp config.gypi
	$(PYTHON) deps/deps.py -f make

config.gypi: configure
	if [ -f $@ ]; then
		$(error Stale $@, please re-run ./configure)
	else
		$(error No $@, please run ./configure first)
	fi

test:
	$(CARGO) test

clean:
	rm -rf ./target.deps
	rm config.*
	rm icu_config.gypi
