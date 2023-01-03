.PHONY: run run-debug watch watch-debug build fmt migrate migrate_test test

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

migrate: 
	sqlx migrate run --database-url postgres://admin:admin@127.0.0.1:35432/postgres --source ./db/migrations

migrate_test:
	sqlx migrate run --database-url postgres://admin:admin@127.0.0.1:35432/postgres_test --source ./db/migrations

test:
	cargo test
