
ifndef ARTIFACTS_DIR
$(error ARTIFACTS_DIR is not set)
endif

# just changing this to be "the lambda i'm trying to write"
LAMBDA=convert

build-HelloRustFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/$(LAMBDA) $(ARTIFACTS_DIR)/bootstrap