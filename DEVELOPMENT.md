# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `make` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [zip](https://linux.die.net/man/1/zip)
* [make](https://www.gnu.org/software/make/)
* [Docker](https://www.docker.com/)

# INSTALL FROM LOCAL SOURCE

```console
$ make install
```

# UNINSTALL

```console
$ make uninstall
```

# LINT

```console
$ make lint
```

# TEST

```console
$ make
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
