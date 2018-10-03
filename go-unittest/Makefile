format:  ## Check format without changes (alias: fmt)
	go vet
.PHONY: format

fmt: | format
.PHONY: fmt

test:  ## Run unit tests
	go test
.PHONY: test

setup:  ## Setup using dep
	dep ensure
.PHONY: setup

clean:  ## Clean up
	go clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[a-z]+:  ## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-10s\033[0m %s\n", $$1, $$2}'
.PHONY: help

.DEFAULT_GOAL = test
default: test
