.POSIX:
.SILENT:
.PHONY: \
	all \
	crates \
	rustup-components

all: crates rustup-components

crates:
	cargo install --force \
		cargo-audit \
		cargo-cache \
		cargo-edit \
		crit@0.0.10 \
		cross@0.2.5 \
		unmake@0.0.21

rustup-components:
	rustup component add \
		clippy \
		rustfmt
