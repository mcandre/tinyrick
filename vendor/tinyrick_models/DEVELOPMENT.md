# OVERVIEW

tinyrick_models' own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/) 1.91.0+
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2

# SECURITY AUDIT

```console
$ make audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ make [build]
```

# PUBLISH

```console
$ make publish
```

# CLEAN

```console
$ make clean
```
