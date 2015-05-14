-include deps/config.mk

BUILDTYPE ?= Release
PYTHON ?= python
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
# just the debug build, run `make -C out BUILDTYPE=Debug` instead.
ifeq ($(BUILDTYPE),Release)
all: out/Makefile $(RUSTJS_EXE)
else
all: out/Makefile $(RUSTJS_EXE) $(RUSTJS_G_EXE)
endif

$(RUSTJS_EXE): config.gypi out/Makefile
	$(MAKE) -C out BUILDTYPE=Release V=$(V)
	ln -fs out/Release/$(RUSTJS_EXE) $@

$(RUSTJS_G_EXE): deps/config.gypi deps/out/Makefile
	$(MAKE) -C out BUILDTYPE=Debug V=$(V)
	ln -fs out/Debug/$(RUSTJS_EXE) $@

deps/out/Makefile: deps/common.gypi deps/v8/build/toolchain.gypi deps/v8/build/features.gypi deps/v8/tools/gyp/v8.gyp deps/deps.gyp config.gypi
	$(PYTHON) deps/deps.py -f make

config.gypi: configure
	if [ -f $@ ]; then
		$(error Stale $@, please re-run ./configure)
	else
		$(error No $@, please run ./configure first)
	fi

clean:
	rm -rf ./out
	rm config.*
	rm icu_config.gypi
