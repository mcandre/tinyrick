# DEVELOPMENT

tinyrick_macros follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [asdf](https://asdf-vm.com/)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
make
```

## Security Audit

```sh
make audit
```

## Lint

```sh
make lint
```

## Test

```sh
make test
```

## Publish Crate

```sh
make publish
```

## Clean Workspace

```sh
make clean
```
