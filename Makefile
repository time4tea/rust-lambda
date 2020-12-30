
ARCH=x86_64-unknown-linux-musl

ARTIFACTS_DIR?=.aws-sam/build/HelloRustFunction
BOOTSTRAP=$(ARTIFACTS_DIR)/bootstrap

# just changing this to be "the lambda i'm trying to write"
LAMBDA=convert

build-HelloRustFunction:
	cargo build --release --target $(ARCH)
	cp ./target/$(ARCH)/release/$(LAMBDA) $(BOOTSTRAP)


.PHONY: all
all: build-HelloRustFunction

.PHONY: watch
watch:
	while true; \
	do \
		inotifywait -r src; \
		$(MAKE) all; \
	done
