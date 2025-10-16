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
		unmake@0.0.23

	cargo install --force \
		cross \
			--git https://github.com/cross-rs/cross \
			--rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa

rustup-components:
	rustup component add \
		clippy \
		rustfmt
