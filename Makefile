PACKAGE=tinyrick
VERSION=0.0.1

.PHONY: $(PACKAGE)-$(VERSION).zip

BIN=target/debug/$(PACKAGE)

all: test

test: install
	sh -c 'cd example && VERBOSE=1 tinyrick'

install: src/lib.rs src/tinyrick.rs
	cargo install --force --path .

uninstall:
	cargo uninstall tinyrick

publish:
	cargo publish

target/x86_64-unknown-linux-gnu/release/tinyrick:
	sh crosscompile-linux.sh x86_64 gnu

target/x86_64-unknown-linux-musl/release/tinyrick:
	sh crosscompile-linux.sh x86_64 musl

crosscompile: target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick

$(PACKAGE)-$(VERSION).zip: crosscompile
	zip $(PACKAGE)-$(VERSION).zip target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick

port: $(PACKAGE)-$(VERSION).zip

clippy:
	cargo clippy

lint: clippy

clean-example:
	-rm -rf example/target
	-rm -rf example/Cargo.lock

clean-cargo:
	-cargo clean

clean-cargo-lock:
	-rm Cargo.lock

clean-ports:
	-rm *.zip

clean: clean-example clean-cargo clean-cargo-lock clean-ports
