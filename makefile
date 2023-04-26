.PHONY: all lint build install test clean

BANNER=tinyrick-0.0.11

all: build

test: install
	@sh -c "cd example && tinyrick -l"
	@sh -c "cd example && tinyrick -v"
	@sh -c "cd example && tinyrick -h"
	@sh -c "cd example && tinyrick"
	@sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build_debug build_release build doc install unit_test integration_test test banner uninstall clean_cargo clean"

install_binaries:
	@cargo install --force --path .

install: install_binaries

uninstall:
	@cargo uninstall tinyrick

audit:
	@cargo audit

doc:
	@cargo doc

clippy:
	@cargo clippy

rustfmt:
	@cargo fmt

unmake:
	@unmake makefile
	@unmake install.mk

lint: doc clippy rustfmt unmake

build: lint test
	@cargo build --release

publish:
	@cargo publish

crit:
	@crit -b $(BANNER)

port: crit
	@sh -c "cd .crit/bin && zip -r $(BANNER).zip $(BANNER)"

clean_example:
	-rm -rf example/target
	-rm -rf example/Cargo.lock

clean_cargo:
	-cargo clean

clean_crit:
	-crit -c

clean: clean_example clean_cargo clean_crit
