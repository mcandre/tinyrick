#!/bin/sh
#
# Avoid make in order to work around https://github.com/rust-lang/rust/issues/52801

PACKAGE='tinyrick'
VERSION='0.0.6'
ARCHIVE="${PACKAGE}-${VERSION}.zip"
BIN="target/debug/$PACKAGE"

test() {
		install &&
				sh -c "cd example && tinyrick" &&
				sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build doc install uninstall unit_test integration_test banner clean_cargo clean"
}

install() {
		cargo install --force --path .
}

uninstall() {
		cargo uninstall tinyrick
}

doc() {
		cargo doc
}

publish() {
		cargo publish
}

crosscompile() {
		sh crosscompile-linux.sh x86_64 gnu &&
				sh crosscompile-linux.sh x86_64 musl
}

port() {
		crosscompile &&
				zip "$ARCHIVE" \
						target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick
}

clippy() {
		cargo clippy
}

lint() {
		clippy
}

clean_example() {
		rm -rf example/target;
				rm -rf example/Cargo.lock
}

clean_cargo() {
		cargo clean
}

clean_ports() {
		rm *.zip
}

clean() {
		clean_example;
				clean_cargo
				clean_ports
}

if [ -z "$1" ]; then
		test
fi

for task in "$@"; do
		"$task"
done
