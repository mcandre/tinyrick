# tinyrick: a freeform Rust build system

```
       .---.              ^
     o{__ω__ o{          ^0^  -Let me out!
~~ ( // *|* \xx\)      xx`|'
        = =  xxxx&x      ' `
```

tinyrick provides a Rust-source task dependency configuration system.

# EXAMPLE

```console
$ cd example

$ tinyrick
running 1 test
test smoketest ... ok
```

# CRATE

https://crates.io/crates/tinyrick

# API DOCUMENTATION

https://docs.rs/tinyrick/

# RUNTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)

# SETUP

Write your own special snowflake `rick.rs` scripts to rule Rust projects, ya supergenius! Mere geniuses can browse the fully wired up [example/](example) project as a working reference.

You will change the `tinyrick` dependency from `tinyrick = { version = "*", path = "..", optional = true }` to like `tinyrick = { version = "<some-pinned-version>", optional = true }` in your `Cargo.toml`, of course. Unless you *want* your build to break, duh!

# WHY TINY RICK?

make is old as farts and encourages vendor locking by expressing tasks primarily as shell commands, not to mention the plethora of competing make syntaxes and slow build times for projects of any complexity. cmake is better but not much. shake is a quasi-robust Haskell-based tool that adds a frickin supercluster-size technology stack to your project, and is self-defeated by its limited DSL. maven is just moronic. In contrast, tinyrick lets you write your tasks as ordinary Rust functions, so you get robustness and flexibility at the same time. Best of both craps, whatever.

# CONTRIBUTING

For more details on developing tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# CREDITS

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects
