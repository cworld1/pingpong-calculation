ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

# Get the platform
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
	@cd lib/hello && cargo build --release
	@cp lib/hello/target/release/libhello.$(SO_EXT) lib/
	@if [ "$(PLATFORM)" = "Windows" ]; then \
		cp lib/hello/target/release/hello.dll build/ ; \
	fi
	go build -ldflags="-r $(ROOT_DIR)build/lib" -o build/ main_dynamic.go

.PHONY: build-static
build-static:
	@cd lib/hello && cargo build --release
	@cp lib/hello/target/release/libhello.a lib/
	go build -o build/ main_static.go

.PHONY: run-dynamic
run-dynamic: build-dynamic
	@./build/main_dynamic

.PHONY: run-static
run-static: build-static
	@./build/main_static

# This is just for running the Rust lib tests natively via cargo
.PHONY: test-rust-lib
test-rust-lib:
	@cd lib/hello && cargo test -- --nocapture

.PHONY: clean
clean:
	rm -rf build/main_dynamic build/main_static lib/libhello.$(SO_EXT) lib/libhello.a lib/hello/target
