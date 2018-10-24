# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.29.2+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [tinyrick](https://github.com/mcandre/tinyrick)

# INSTALL FROM LOCAL SOURCE

```console
$ tinyrick install
```

# UNINSTALL

```console
$ tinyrick uninstall
```

# LINT

```console
$ tinyrick lint
```

# UNIT + INTEGRATION TEST

```console
$ tinyrick test
```

# UNIT TEST

```console
$ tinyrick unit_test
```

# INTEGRATION TEST

```console
$ tinyrick integration_test
```

# CLEAN

```console
$ tinyrick clean
```
