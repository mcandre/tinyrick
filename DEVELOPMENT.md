# DEVELOPMENT GUIDE

tinyrick follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)

# COMPILE AND INSTALL

```sh
make install
```

# UNINSTALL

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

# CLEAN

```sh
make clean
```
