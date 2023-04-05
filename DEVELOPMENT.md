# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.68.2+ with `rustup component add clippy rustfmt` and `cargo install cargo-audit@0.17.5 crit@0.0.6`
* [Go](https://go.dev/) 1.20.2+ with `go install github.com/mcandre/accio/cmd/accio@v0.0.4` and `accio -install`
* [make](https://pubs.opengroup.org/onlinepubs/009695299/utilities/make.html)
* a UNIX environment with [coreutils](https://www.gnu.org/software/coreutils/) / [base](http://ftp.freebsd.org/pub/FreeBSD/releases/) / [macOS](https://www.apple.com/macos) / [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) / etc.
* GNU compatible [findutils](https://www.gnu.org/software/findutils/)
* [Docker](https://www.docker.com/) 20.10.21+
* [zip](https://linux.die.net/man/1/zip)

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ make install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ make uninstall
```

# AUDIT

```console
$ make audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ make build
```

# PUBLISH

```console
$ make publish
```

# PORT

```console
$ make port
```

# CLEAN

```console
$ make clean
```
