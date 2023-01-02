build:
	@cargo clean
	@cargo build --release

filter:
	@cargo run --bin filter

test:
	@cargo test

fix:
	@cargo fmt
	@cargo fix --allow-dirty --allow-staged

