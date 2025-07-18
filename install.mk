.POSIX:
.SILENT:
ALLTARGETS!=ls -a
.PHONY: $(ALLTARGETS)

all: crates rustup-components

crates:
	cargo install --force \
		cargo-audit \
		crit@0.0.9 \
		cross@0.2.5 \
		unmake@0.0.21

rustup-components:
	rustup component add \
		clippy \
		rustfmt
