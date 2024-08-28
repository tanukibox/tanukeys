.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: dev/deps
dev/build: dev/deps
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: deps
deps:
	cargo update

.PHONY: dev/deps
dev/deps:
	cargo install --force cargo-watch
	cargo install --force cargo-make

.PHONY: format
format:
	cargo fmt
