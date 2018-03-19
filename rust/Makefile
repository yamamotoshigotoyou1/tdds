format:
	cargo fmt --all -- --write-mode=diff
.PHONY: format

fmt: | format
.PHONY: fmt

test:
	cargo test
.PHONY: test

coverage:
	../tools/build-kcov
	cargo build --tests
	../tools/check-kcov money_test
.PHONY: coverage

cov: | coverage
.PHONY: cov

clean:
	cargo clean
.PHONY: clean

.DEFAULT_GOAL = test
default: test
