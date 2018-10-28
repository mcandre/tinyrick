# tinyrick: a freeform Rust build system

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
```

# ABOUT

I'm tinyrick (TINYRICK!) and I build Rust projects. With tinyrick, you configure your build in the same normal Rust code as the rest of your project. Or keep picking your nose with make, it's up to you.

Look at my pants! tinyrick! You think my pants are one size fits all? No, of course not! So get the pants that fit you. Get a tiny `rick.rs` that fits your workflow. Task dependency trees, get em while they're hot! Segfaults, get em while they're not. Smarter, butter, faster, stranger.

Don't shell out, lib out. Your build is more portable that way. Holy Maven, put that foot on some ice! Who knows, maybe fair market feet prices will go up next year! tinyricktinyricktinyrick. If you look closely, that last period is actually a *micro* rick rendered in ASCII; even tinier tinyrick!

# CRATE

https://crates.io/crates/tinyrick

# API DOCUMENTATION

https://docs.rs/tinyrick/

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+

# SETUP

## rick.rs

Write some tasks in a `rick.rs` build configuration script at the top-level directory of your Rust project:

```rust
fn test() {
  tinyrick::exec!("cargo", &["test"]);
}

fn build() {
  tinyrick::deps(build);
  tinyrick::exec!("cargo", &["build", "--release"]);
}

fn publish() {
  tinyrick::exec!("cargo", &["publish"]);
}

fn clean() {
  tinyrick::exec!("cargo", &["clean"]);
}

fn main() {
  tinyrick::phony!(clean);
  tinyrick::wubba_lubba_dub_dub!(build; test, publish, clean);
}
```

## Cargo.toml

Now, wire up your project's `tinyrick` command line interface by configuring your top-level `Cargo.toml`:

```toml
[package]
name = "derpmobile"
description = "hyperadvanced derpmobiles"
version = "3.1.4"

[dependencies]
tinyrick = { version = "0.0.6", optional = true }

[features]
letmeout = ["tinyrick"]

[[bin]]
name = "rick"
path = "rick.rs"
required-features = ["letmeout"]
```

Give `tinyrick` a whirl and watch how he behaves? I hope he's practicing good manners :P

What happens when you run:

* `tinyrick test`?
* `tinyrick publish`?
* `tinyrick clean`?
* `tinyrick build`?

I bet the freakin' moon explodes if you run `tinyrick build build build`! (Hold onto your pants~)

### Debriefing

Where are my pants? Let's break down the code so far:

* `fn name() { ... }` declares a task named `name`.
* `deps(requirement)` caches a dependency on task `requirement`.
* `exec!(...)` runs raw shell commands.
* `phony!(...)` disables dependency caching for some tasks.
* `wubba_lubba_dub_dub!(default; ...)` exposes a `default` task and some other tasks to the `tinyrick` command line.
* `letmeout` is a feature gate, so that neither the `tinyrick` package, nor the mini `rick` binary escape with your Rust package when you `tinyrick publish`.

### DoN't UsE sHelL cOmMaNdS!1

Just because the tinyrick library offers you several *supremely convenient* macros for executing shell commands doesn't mean that you should always shell out. No way! Whenever possible, use regular Rust code. There's like a ba-jillion [crates](https://crates.io) of prewritten code, so you might as well use 'em.

# CONTRIBUTING

For more details on developing tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# CREDITS

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects

# EVEN MORE EXAMPLES

* The included [example](example) project provides a fully qualified demonstration of how to build projects with tinyrick.
* For a more practical example, see [ios7crypt-rs](https://github.com/mcandre/ios7crypt-rs), a little *modulino* library + command line tool for *deliciously dumb* password encryption.
* [tinyrick_extras](https://github.com/mcandre/tinyrick_extras) defines some common workflow tasks as plain old Rust functions, that you can sprinkle onto your tinyrick just like any other Rust crate.
