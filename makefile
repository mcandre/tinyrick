.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-cargo \
	clean-example \
	clippy \
	crit \
	doc \
	install \
	lint \
	publish \
	rustfmt \
	test \
	uninstall
.IGNORE: \
	clean \
	clean-cargo \
	clean-example

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-cargo \
	clean-example

clean-cargo:
	cargo clean

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	rm -rf example/.crit

clippy:
	cargo clippy

doc:
	cargo doc

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

publish:
	cargo publish

rustfmt:
	cargo fmt

test: install
	sh -c "cd example && tinyrick -l"
	sh -c "cd example && tinyrick -v"
	sh -c "cd example && tinyrick -h"
	sh -c "cd example && tinyrick"
	sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build_debug build_release build doc install unit_test integration_test test uninstall clean_cargo clean"

uninstall:
	cargo uninstall tinyrick
