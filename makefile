.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports \
	clippy \
	crit \
	doc \
	docker-build \
	docker-push \
	docker-test \
	install \
	lint \
	port \
	publish \
	rustfmt \
	test \
	uninstall
.IGNORE: \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

VERSION=0.0.23
BANNER=tinyrick-$(VERSION)

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

clean-archive:
	rm ".crit/bin/$(BANNER).tgz"

clean-cargo:
	cargo clean

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target

clean-ports:
	crit -c

clippy:
	cargo clippy

CRIT_EXCLUSIONS=android|cuda|emscripten|fortanix|fuchsia|gnullvm|gnux32|ios|loongarch|msvc|none-eabi|ohos|pc-solaris|powerpc64le-unknown-linux-musl|redox|riscv64gc-unknown-linux-musl|sparcv9-sun-solaris|uefi|unknown-none|wasm|i686-pc-windows-gnu

crit:
	crit -b $(BANNER) -e "$(CRIT_EXCLUSIONS)"

doc:
	cargo doc

docker-build:
	tuggy -t n4jm4/tinyrick:$(VERSION) --load

docker-push:
	tuggy -t n4jm4/tinyrick:$(VERSION) -a n4jm4/tinyrick --push

docker-test:
	tuggy -t n4jm4/tinyrick:test --load --push

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

port: crit
	chandler -C .crit/bin -czf "$(BANNER).tgz" "$(BANNER)"

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
