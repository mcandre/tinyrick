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
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
 Running `target/debug/tinyrick`
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
 Running target/debug/deps/arithmancy-55e3b9e6f9c67629

running 1 test
test smoketest ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 Running target/debug/deps/add_two-eb73f737580379e1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests arithmancy

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Finished dev [unoptimized + debuginfo] target(s) in 0.05s
```

See the [example/](example) project for more configuration details.

# RUNTIME REQUIREMENTS

* [cargo](https://www.rust-lang.org/en-US/)

# CONTRIBUTING

For more details on developing cargo-tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# CREDITS

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects
