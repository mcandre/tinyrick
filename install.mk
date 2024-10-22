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
		crit@0.0.7 \
		cross@0.2.5 \
		unmake@0.0.16

rustup-components:
	rustup component add \
		clippy \
		rustfmt
