build:
	@dotnet build ./src/Money.sln
.PHONY: build

clean:
	@dotnet clean ./src/Money.sln
.PHONY: clean

format:
	@dotnet format --verify-no-changes ./src/Money.sln
.PHONY: format

fmt: format
.PHONY: fmt

test:
	@dotnet test ./src/Money.sln
.PHONY: test
