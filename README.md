# tinyrick: a freeform Rust build system

[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/tinyrick?label=crate%20downloads)](https://crates.io/crates/tinyrick) [![docs.rs](https://img.shields.io/docsrs/tinyrick)](https://docs.rs/tinyrick/latest/tinyrick/) [![Test](https://github.com/mcandre/tinyrick/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/tinyrick/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

```txt
       .---.              ^
     o{__ω__ o{          ^0^  -Let me out!
~~ ( // *|* \xx\)      xx`|'
        = =  xxxx&x      ' `
```

# SUMMARY

tinyrick is a framework for writing development tasks as Rust code.

# EXAMPLE

```console
% cd example

% tinyrick
running 1 test
test smoketest ... ok

% tinyrick -h
Usage: tinyrick [options]

Options:
    -l, --list          list available tasks
    -h, --help          print usage info
    -v, --version       print version info
```

# DOWNLOAD

```sh
cargo install tinyrick
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

## Recommended

* [tinyrick_extras](https://github.com/mcandre/tinyrick_extras)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

For details on tuning tinyrick, see [CONFIGURATION](CONFIGURATION.md).

For details on building from source, see [DEVELOPMENT](DEVELOPMENT.md).

# RESOURCES

Prior art, personal plugs, and tools for developing portable applications (including non-Rust projects)!

* [Gradle](https://gradle.org/) - JVM build system
* [Rake](https://ruby.github.io/rake/) - Ruby task runner
* [Rebar3](https://www.rebar3.org/) - Erlang task runner
* [Shake](https://shakebuild.com/) - Haskell task runner
* [invoke](https://pypi.org/project/invoke/) - Python task runner
* [lake](https://luarocks.org/modules/steved/lake) - Lua task runner
* [mage](https://magefile.org/) - Go task runner
* POSIX [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html) standard
* [mcandre/booty](https://github.com/mcandre/booty) - JS task runner pattern

🛸
