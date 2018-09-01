check:  ## Check rust syntax
	cargo check --all -v
.PHONY: check

format:  ## Check format without changes (alias: fmt)
	cargo fmt --all -- --check
.PHONY: format

fmt: | format
.PHONY: fmt

test:  ## Run unit tests
	cargo test
.PHONY: test

coverage:  ## Generate coverage report of unit tests using kcov (alias: cov)
	## ../.tools/build-kcov
	cargo build --tests
	../.tools/check-kcov money_test kcov
.PHONY: coverage

cov: | coverage
.PHONY: cov

clean:  ## Clean up
	cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[a-z]+:  ## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-10s\033[0m %s\n", $$1, $$2}'
.PHONY: help

.DEFAULT_GOAL = test
default: test
