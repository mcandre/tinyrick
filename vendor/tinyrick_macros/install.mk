.POSIX:
.SILENT:
.PHONY: all

all:
	cargo install --force \
		cargo-audit \
		cargo-cache \
		cargo-edit \
		unmake@0.0.23
	rustup component add \
		clippy \
		rustfmt
