clean:
	cargo clean
.PHONY: clean

test:
	cargo test
.PHONY: test

coverage:
	../tools/build-kcov
	cargo test
	../tools/check-kcov money
.PHONY: coverage

cov: | coverage
.PHONY: cov

.DEFAULT_GOAL = test
default: test
