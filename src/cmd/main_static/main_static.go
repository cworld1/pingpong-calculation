package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: ../../../lib/libpingpong.a -ldl
#include "../../../lib/pingpong.h"
*/
import "C"

// noinspection DuplicateDecl
func main() {
	C.pingpong(C.CString("world"))
	C.whisper(C.CString("this is code from the static library"))
}
