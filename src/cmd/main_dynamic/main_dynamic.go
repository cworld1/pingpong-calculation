package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../../lib -lpingpong
#include "../../../lib/pingpong.h"
*/
import "C"
import "fmt"

func main() {
	// C.pingpong(C.CString("world"))
	var result = C.GoString(C.get_best_action(C.CString("SB_2")))
	fmt.Println(result)
	C.whisper(C.CString("(this is code from the dynamic library)"))

}
