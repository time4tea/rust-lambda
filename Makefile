
ifndef ARTIFACTS_DIR
$(error ARTIFACTS_DIR is not set)
endif


build-HelloRustFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/hello $(ARTIFACTS_DIR)/bootstrap