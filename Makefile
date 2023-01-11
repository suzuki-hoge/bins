build:
	@cargo clean
	@cargo build --release

filter:
	@cargo run --bin filter

l:
	@cargo run --bin command-launcher

test:
	@cargo test

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged

