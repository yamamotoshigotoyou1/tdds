package main

import (
	"fmt"
	"reflect"
)

// TestCase ...
type TestCase interface {
	Run(tt TestCase)
}

// TestRunner ...
type TestRunner struct {
	Name string
}

// Run a test case
//
// Is it possible to run this without tt ?
// I have to do like this, right now:
//
// ```
// test.Run(&test)
//
// # but I want
// test.Run()
// ```
func (t *TestRunner) Run(tt TestCase) {
	v := reflect.ValueOf(t).Elem()
	name := fmt.Sprintf("%v", v.FieldByName("Name"))

	// new WasRun object
	nv := reflect.New(reflect.TypeOf(tt).Elem())
	method := nv.MethodByName(name)
	if method.IsValid() {
		args := []reflect.Value{}
		method.Call(args)
	}

	// update WasRun.WasRun
	vv := reflect.ValueOf(tt).Elem()
	wasRun := vv.FieldByName("WasRun")
	if wasRun.Kind() == reflect.Int && wasRun.CanSet() {
		wasRun.SetInt(1)
	}
}

// WasRun ...
type WasRun struct {
	*TestRunner
	WasRun int
}

// NewWasRun ...
func NewWasRun(name string) WasRun {
	return WasRun{
		&TestRunner{Name: name},
		0,
	}
}

// TestMethod ...
func (r *WasRun) TestMethod() {
	// do something
}

func main() {
	test := NewWasRun("TestMethod")
	test.Run(&test)
}
