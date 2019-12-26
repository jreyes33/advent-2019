.ONESHELL:
.PHONY: check clean build fmt clippy test

check: clean build fmt clippy test

clean:
	cd rust
	cargo clean

build:
	cd rust
	cargo build --verbose --examples

fmt:
	cd rust
	cargo fmt -- --check

clippy:
	cd rust
	cargo clippy --examples -- --forbid warnings

test:
	cd rust
	cargo test --verbose --all-targets
