
all: generate build
.PHONY: all

build:
	cargo build
.PHONY: build

generate:
	./scripts/generate.sh
	./scripts/errors/generate-errors.sh
.PHONY: generate
