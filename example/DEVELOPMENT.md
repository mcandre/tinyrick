# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `cargo tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)

# INSTALL FROM LOCAL SOURCE

```console
$ cargo tinyrick install
```

# UNINSTALL

```console
$ cargo tinyrick uninstall
```

# LINT

```console
$ cargo tinyrick lint
```

# UNIT + INTEGRATION TEST

```console
$ cargo tinyrick test
```

# UNIT TEST

```console
$ cargo test
```

# INTEGRATION TEST

```console
$ cargo tinyrick integration_test
```

# CLEAN

```console
$ cargo clean
```
