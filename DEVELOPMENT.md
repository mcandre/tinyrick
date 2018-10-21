# OVERVIEW

cargo-tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `make` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [zip](https://linux.die.net/man/1/zip)
* [make](https://www.gnu.org/software/make/)
* [Docker](https://www.docker.com/)

# INSTALL FROM LOCAL SOURCE

```
$ make install
```

# UNINSTALL

```
$ make uninstall
```

# LINT

```
$ make lint
```

# TEST

```
$ make
```

# PORT

```
$ make cargo-tinyrick-0.0.1.zip
```

# CLEAN

```
$ make clean
```
