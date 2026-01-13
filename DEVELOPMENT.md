# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 20.10.21+
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```sh
make install
```

# UNINSTALL BINARY ARTIFACTS

```sh
make uninstall
```

# SECURITY AUDIT

```sh
make audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```sh
make build
```

# PUBLISH

```sh
make publish
```

# PORT

```sh
make port
```

# TEST DOCKER IMAGES

```sh
make docker-test
```

# PUSH DOCKER IMAGES

```sh
make docker-push
```

# CLEAN

```sh
make clean
```
