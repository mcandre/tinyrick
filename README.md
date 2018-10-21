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

$ cargo tinyrick
running 1 test
test smoketest ... ok
```

# RUNTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)

# SETUP

Write your own special snowflake `tinyrick.rs` scripts to rule Rust projects, ya supergenius! Mere geniuses can browse the fully wired up [example/](example) project as a working reference.

# CONTRIBUTING

For more details on developing cargo-tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# CREDITS

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects
