# OVERVIEW

arithmancy's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `cargo tinyrick` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)

# INSTALL FROM LOCAL SOURCE

```
$ cargo tinyrick install
```

# UNINSTALL

```
$ cargo tinyrick uninstall
```

# LINT

```
$ cargo tinyrick lint
```

# UNIT + INTEGRATION TEST

```
$ cargo tinyrick test
```

# UNIT TEST

```
$ cargo test
```

# INTEGRATION TEST

```
$ cargo tinyrick integration_test
```

# CLEAN

```
$ cargo clean
```
