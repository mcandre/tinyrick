# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)

# INSTALL ARTIFACTS FROM LOCAL SOURCE

```console
$ tinyrick install
```

# UNINSTALL ARTIFACTS

```console
$ tinyrick uninstall
```

# LINT

```console
$ tinyrick lint
```

# RUN ALL TESTS

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

# GENERATE DOCUMENTATION

```console
$ tinyrick doc
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
