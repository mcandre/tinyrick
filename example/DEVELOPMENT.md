# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)

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
