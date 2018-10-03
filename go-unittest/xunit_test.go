package main

import (
	"testing"

	"github.com/stretchr/testify/suite"
)

// TestCaseTest ...
type TestCaseTest struct {
	suite.Suite
}

// TestRunnig ...
func (suite *TestCaseTest) TestRunnig() {
	test := NewWasRun("TestMethod")
	suite.Equal(0, test.WasRun)
	test.Run(&test)
	suite.Equal(1, test.WasRun)
}

func TestTestCaseTest(t *testing.T) {
	suite.Run(t, new(TestCaseTest))
}
