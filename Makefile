PACKAGE=$(shell cargo -Z unstable-options build --build-plan | jq -r ".invocations[0].package_name")
VERSION=$(shell cargo -Z unstable-options build --build-plan | jq -r ".invocations[0].package_version")

ARCHIVE=$(PACKAGE)-$(VERSION).zip

.PHONY: $(ARCHIVE)

BIN=target/debug/$(PACKAGE)

all: test

test: install
	sh -c "cd example && tinyrick"
	sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build doc install uninstall unit_test integration_test banner clean_cargo clean"

install: src/lib.rs src/tinyrick.rs
	cargo install --force --path .

uninstall:
	cargo uninstall tinyrick

doc:
	cargo doc

publish:
	cargo publish

target/x86_64-unknown-linux-gnu/release/tinyrick:
	sh crosscompile-linux.sh x86_64 gnu

target/x86_64-unknown-linux-musl/release/tinyrick:
	sh crosscompile-linux.sh x86_64 musl

crosscompile: target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick

$(ARCHIVE): crosscompile
	zip $(ARCHIVE) target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick

port: $(ARCHIVE)

clippy:
	cargo clippy

lint: clippy

clean-example:
	-rm -rf example/target
	-rm -rf example/Cargo.lock

clean-cargo:
	-cargo clean

clean-ports:
	-rm *.zip

clean: clean-example clean-cargo clean-ports
