package main

import (
	"fmt"
	"reflect"
)

// WasRun ...
type WasRun struct {
	name string
	wasRun int
}

func wasRun(name string) WasRun {
	return WasRun{
		name: name,
		wasRun: 0,
	}
}

// TestMethod ...
func (r *WasRun) TestMethod() {
	r.wasRun = 1
}

func (r *WasRun) run() {
	t := reflect.ValueOf(r)

	// reflect is available only for exported methods
	method := t.MethodByName(r.name)
	if method.IsValid() {
		args := []reflect.Value{}
		method.Call(args)
	}
}


func main() {
	test := wasRun("TestMethod")
	fmt.Println(test.wasRun)
	test.run()
	fmt.Println(test.wasRun)
}
