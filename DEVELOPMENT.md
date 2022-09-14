# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.63+
* [cargo-audit](https://crates.io/crates/cargo-audit) 0.16
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [zip](https://linux.die.net/man/1/zip)
* [Docker](https://www.docker.com/)

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ sh build install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ sh build uninstall
```

# SECURITY AUDIT

```console
$ sh build audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ sh build
```

# PUBLISH

```console
$ sh build publish
```

# PORT

```console
$ sh build port
```

# CLEAN

```console
$ sh build clean
```
