# cargo

build:
	@cargo clean
	@cargo build --release

test:
	@cargo test

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo test
	@cargo run --bin document-gatherer

# debug

bf:
	@cargo run --bin bf

bb:
	@cargo run --bin bb
