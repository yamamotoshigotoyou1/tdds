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

// WasRun ...
type WasRun struct {
	Name   string
	WasRun int
}

// NewWasRun ...
func NewWasRun(name string) WasRun {
	return WasRun{
		Name:   name,
		WasRun: 0,
	}
}

// TestMethod ...
func (r *WasRun) TestMethod() {
	// do something
}
