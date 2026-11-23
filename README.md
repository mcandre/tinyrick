# tinyrick: a freeform Rust build system

[![Crate Downloads](https://badgen.net/crates/d/tinyrick?label=crate+downloads&icon=rust)](https://crates.io/crates/tinyrick) [![API Docs](https://badgen.net/static/docs/API/3eca2a?icon=docs.rs)](https://docs.rs/tinyrick/latest/tinyrick/) [![CI/CD](https://badgen.net/github/checks/mcandre/tinyrick?icon=github)](https://github.com/mcandre/tinyrick/actions) [![License](https://badgen.net/static/license/BSD/ab2b29?icon=freebsd)](LICENSE.md) [![AI](https://badgen.net/static/ai/Gemini|hint?color=886fbf&icon=google-gemini)](AI-DECLARATION.md)

```text
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
