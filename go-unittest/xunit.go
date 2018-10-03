package main

import (
	"fmt"
	"reflect"
)

// TestCase ...
type TestCase interface {
}

func run(t TestCase) {
	v := reflect.ValueOf(t).Elem()
	name := fmt.Sprintf("%v", v.FieldByName("Name"))

	// new object
	vv := reflect.New(v.Type())

	method := vv.MethodByName(name)
	if method.IsValid() {
		args := []reflect.Value{}
		method.Call(args)
	}

	wasRun := v.FieldByName("WasRun")
	if wasRun.Kind() == reflect.Int {
		wasRun.SetInt(1)
	}
}

// MyTestCase ...
type MyTestCase struct {
	Name   string
	WasRun int
}

// NewMyTestCase ...
func NewMyTestCase(name string) MyTestCase {
	return MyTestCase{
		Name:   name,
		WasRun: 0,
	}
}

// TestMethod ...
func (r *MyTestCase) TestMethod() {
	// do something
	//fmt.Println("TestMethod")
}

func main() {
	test := NewMyTestCase("TestMethod")
	fmt.Println(test.WasRun)
	run(&test)
	fmt.Println(test.WasRun)
}
