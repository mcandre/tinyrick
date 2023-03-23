# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.63+
* [cargo-audit](https://crates.io/crates/cargo-audit) 0.16
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [Go](https://go.dev/) 1.20.2+ with `go install github.com/mcandre/accio/cmd/accio@v0.0.4` and `accio -install`
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [zip](https://linux.die.net/man/1/zip)
* [Docker](https://www.docker.com/) 20.10.21+

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2

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
