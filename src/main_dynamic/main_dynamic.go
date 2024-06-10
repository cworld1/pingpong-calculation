package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L../../lib -lpingpong
#include "../../lib/pingpong.h"
*/
import "C"
import (
	"encoding/json"

	"github.com/gin-gonic/gin"
)

func main() {
	C.whisper(C.CString("(this is code from the dynamic library)"))
	start_server()
}

func start_server() {
	r := gin.Default()

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})

	r.GET("/best_action", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": best_action(),
		})
	})

	r.Run() // listen and serve on
}

func best_action() map[string]interface{} {
	var result = C.GoString(C.get_best_action(C.CString("SB_2")))
	// fmt.Println(result)
	var format_result map[string]interface{}
	json.Unmarshal([]byte(result), &format_result)
	return format_result
}
