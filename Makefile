build:
	@cargo clean
	@cargo build --release

filter:
	@cargo run --bin filter

test:
	@cargo test

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
