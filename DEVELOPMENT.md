# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 20.10.21+
* [GNU make](https://www.gnu.org/software/make/) 3+
* [Rust](https://www.rust-lang.org/en-US/)
* [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/) compatible [tar](https://en.wikipedia.org/wiki/Tar_(computing))
* Provision additional dev tools with `make -f install.mk [-j 4 --output-sync]`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [tar](https://en.wikipedia.org/wiki/Tar_(computing))

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ make install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ make uninstall
```

# SECURITY AUDIT

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
