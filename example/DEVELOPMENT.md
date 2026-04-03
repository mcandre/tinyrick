# DEVELOPMENT

We follow standard, `cargo` based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# PREREQUISITES

* [Rust](https://www.rust-lang.org/en-US/)
* [tinyrick](https://github.com/mcandre/tinyrick)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
tinyrick
```

## Install

```sh
tinyrick install
```

## Uninstall

```sh
tinyrick uninstall
```

## Security Audit

```sh
tinyrick audit
```

## Lint

```sh
tinyrick lint
```

## Test

```sh
tinyrick test
```

## Publish Crate

```sh
tinyrick publish
```

## Clean Workspace

```sh
tinyrick clean
```
