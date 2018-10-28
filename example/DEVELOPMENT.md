# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ tinyrick install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ tinyrick uninstall
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ tinyrick [build]
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
