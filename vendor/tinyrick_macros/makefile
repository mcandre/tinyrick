.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-cargo \
	clean-ports \
	clippy \
	doc \
	install \
	lint \
	publish \
	rustfmt \
	test \
	unmake
.IGNORE: \
	clean \
	clean-cargo

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-cargo

clean-cargo:
	cargo clean

clippy:
	cargo clippy

doc:
	cargo doc

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt \
	unmake

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

unmake:
	unmake .
	unmake -n .
