ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

# Determine the platform
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Linux)
	PLATFORM = Linux
	SO_EXT = so
endif
ifeq ($(UNAME_S), Darwin)
	PLATFORM = macOS
	SO_EXT = dylib
endif
ifneq ($(findstring $(UNAME_S), CYGWIN MSYS_NT),)
	PLATFORM = Windows
	SO_EXT = dll.a
endif
ifeq ($(SO_EXT),)
	PLATFORM = Windows
	SO_EXT = dll.a
endif

# PHONY means that it doesn't correspond to a file; it always runs the build commands.

.PHONY: build-all
build-all: build-dynamic build-static

.PHONY: run-all
run-all: run-dynamic run-static

.PHONY: build-dynamic
build-dynamic:
	@cd lib/pingpong && cargo build --release
	@cp lib/pingpong/target/release/libpingpong.$(SO_EXT) lib/
	@if [ "$(PLATFORM)" = "Windows" ]; then \
		cp lib/pingpong/target/release/pingpong.dll build/ ; \
	fi
	@export GOPATH=$(ROOT_DIR)
	cd src && GOPATH=$(ROOT_DIR) go build -o "$(ROOT_DIR)build/" -ldflags="-r $(ROOT_DIR)build/lib" ./main_dynamic

.PHONY: build-static
build-static:
	@cd lib/pingpong && cargo build --release
	@cp lib/pingpong/target/release/libpingpong.a lib/
	@export GOPATH=$(ROOT_DIR)
	cd src && go build -o "$(ROOT_DIR)build/" ./main_static

.PHONY: run-dynamic
run-dynamic: build-dynamic
	@./build/main_dynamic

.PHONY: run-static
run-static: build-static
	@./build/main_static

# This is just for running the Rust lib tests natively via cargo
.PHONY: test-rust-lib
test-rust-lib:
	@cd lib/pingpong && cargo test -- --nocapture

.PHONY: test-rust-main
test-rust-main:
	@cd lib/pingpong && cargo test -- --nocapture simulated_main_function

.PHONY: clean
clean:
	rm -rf build/main_dynamic build/main_static lib/libpingpong.$(SO_EXT) lib/libpingpong.a lib/pingpong/target
