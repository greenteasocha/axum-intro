.PHONY: run run-debug watch watch-debug build fmt

run:
	 cargo run

run-debug:
	RUST_LOG=debug cargo run

watch:
	cargo watch -x run

watch-debug:
	RUST_LOG=debug cargo watch -x run

build:
	cargo build

fmt:
	cargo fmt
