
build-HelloRustFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/rust-lambda $(ARTIFACTS_DIR)/bootstrap