# tinyrick: a freeform Rust build system

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/tinyrick)](https://hub.docker.com/r/n4jm4/tinyrick) [![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/tinyrick?label=crate%20downloads)](https://crates.io/crates/tinyrick) [![docs.rs](https://img.shields.io/docsrs/tinyrick)](https://docs.rs/tinyrick/latest/tinyrick/) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

```
       .---.              ^
     o{__ω__ o{          ^0^  -Let me out!
~~ ( // *|* \xx\)      xx`|'
        = =  xxxx&x      ' `
```

# EXAMPLE

```console
$ cd example

$ tinyrick
running 1 test
test smoketest ... ok

$ tinyrick -h
Usage: tinyrick [options]

Options:
    -l, --list          list available tasks
    -h, --help          print usage info
    -v, --version       print version info
```

# ABOUT

I'm tinyrick (TINYRICK!) and I build Rust projects. With tinyrick, you configure your build in the same normal Rust code as the rest of your project. Or keep picking your nose with make, it's up to you.

Look at my pants! tinyrick! You think my pants are one size fits all? No, of course not! So get the pants that fit you. Get a `tinyrick.rs` that fits your workflow. Task dependency trees, get em while they're hot! Segfaults, get em while they're not. Smarter, butter, faster, stranger.

Don't shell out, lib out. Your build is more portable that way. tinyricktinyricktinyrick. If you look closely, that last period is actually a *micro* rick rendered in ASCII; even tinier tinyrick!

# INSTALLATION

See [INSTALL.md](INSTALL.md).

## Recommended

* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tinyrick_extras](https://github.com/mcandre/tinyrick_extras)

# SETUP

## tinyrick.rs

Write some tasks in a `tinyrick.rs` build configuration script at the top-level directory of your Rust project:

```rust
use tinyrick::*;

#[default_task]
fn build() {
    deps!(build_release);
}

#[task]
fn build_release() {
    deps!(test);
    exec!("cargo", "build", "--release");
}

#[task]
fn clippy() {
    exec!("cargo", "clippy");
}

// ...

fn main() {
    run()
}
```

## Cargo.toml

Now, wire up the tinyrick command line interface by configuring your top-level `Cargo.toml`:

```toml
[package]
name = "arithmancy"

[dependencies]
ctor = { version = "0.6.2", optional = true }
die = "0.2.0"
tinyrick = { version = "0.0.24", optional = true }
tinyrick_macros = { version = "0.0.2", optional = true }
tinyrick_models = { version = "0.0.2", optional = true }

# ...

[features]
letmeout = [
    "ctor",
    "tinyrick",
    "tinyrick_macros",
    "tinyrick_models",
]

[[bin]]
name = "tinyrick"
path = "tinyrick.rs"
required-features = ["letmeout"]
```

Launch a terminal session in your project directory. Install and run the tinyrick tool:

```console
$ cargo install tinyrick
$ tinyrick
```

Watch how he behaves... I hope tinyrick is practicing good manners :P

What happens when you run:

* `tinyrick test`?
* `tinyrick clean`?
* `tinyrick build`?
* `tinyrick -h`?
* `tinyrick --list`?
* `VERBOSE=1 tinyrick build`?

I bet the freakin' moon explodes if you run `VERBOSE=1 tinyrick build build build`! (Hold onto your pants~)

# DEBRIEFING

Where are my pants? Let's break down the code so far:

* `#[task] fn name() { ... }` declares a task named `name`.
* `#[default_task] fn name() { ... }` declares a task named `name` and marks it as the default, when no CLI arguments are passed to `tinyrick`.
* `deps!(requirement)` caches a dependency on task `requirement`.
* `exec(command, args sequence)` and `exec!(command[, arg[, arg[, arg ...]]])` run CLI commands.
* `VERBOSE=1` enables command string emission during processing.
* `letmeout` is a feature gate, so that neither the tinyrick package, nor your tinyrick binary escape with your Rust package when you `tinyrick publish`.

# DoN't UsE sHelL cOmMaNdS!1

Just because the tinyrick library offers several *supremely convenient* macros for executing shell commands doesn't mean that you should always shell out. No way, man!

Whenever possible, use regular Rust code. There's like a ba-jillion [crates](https://crates.io) of prewritten Rust code, so you might as well use it!

# SEE ALSO

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects
* [bb](https://github.com/mcandre/bb), a build system for (g)awk projects
* [beltaloada](https://github.com/mcandre/beltaloada), a guide to writing build systems for (POSIX) sh
* [booty](https://github.com/mcandre/booty?tab=readme-ov-file) for JS/Node.js/altJS
* [cargo](https://doc.rust-lang.org/cargo/reference/build-scripts.html) custom build scripts, primarily for generating Rust source files from other languages
* [cmake](https://cmake.org/) for C/C++ projects
* [dale](https://github.com/mcandre/dale) builds D projects
* [GNU autotools](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html), a build system for Linux C/C++ projects
* [Gradle](https://gradle.org/), a build system for JVM projects
* [invoke](https://pypi.org/project/invoke/), a Python task runner
* [jelly](https://github.com/mcandre/jelly), a JSON task runner
* [lair](https://github.com/mcandre/lair), a lightweight task runner
* [lake](https://luarocks.org/modules/steved/lake), a Lua task runner
* [Leiningen](https://leiningen.org/) + [lein-exec](https://github.com/kumarshantanu/lein-exec), a Clojure task runner
* [lichen](https://github.com/mcandre/lichen), a sed task runner
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html), the classic, application language agnostic task runner
* [mian](https://github.com/mcandre/mian), a task runner for (Chicken) Scheme Lisp
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [Rebar3](https://www.rebar3.org/), a build system for Erlang projects
* [rez](https://github.com/mcandre/rez) builds C/C++ projects
* [sbt](https://www.scala-sbt.org/index.html), a build system for Scala projects
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [yao](https://github.com/mcandre/yao), a task runner for Common LISP projects

🛸
