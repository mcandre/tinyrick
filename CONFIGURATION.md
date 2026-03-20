# CONFIGURATION

tinyrick declares tasks as Rust code.

# tinyrick.rs

tinyrick looks for a `tinyrick.rs` source file in the current working directory.

Example:

```rust
use tinyrick::*;

#[default_task]
fn build() {
    deps!(test);
    exec!("cargo", "build", "--release");
}

#[task]
fn test() {
    exec!("cargo", "test");
}

// ...

fn main() {
    run()
}
```

## Attributes

tinyrick uses Rust function attributes to register task definitions as `tinyrick <task>` subcommands.

### task

The `task` attribute registers a function as a tinyrick task.

A task function signature has zero arity, and returns no values. For error handling, we recommend the [die](https://crates.io/crates/die) crate.

Example:

```rust
#[task]
fn test() {
    exec!("cargo", "test");
}
```

### default_task

The `default_task` attribute registers a function as the default task, when `tinyrick` runs with no explicit task list.

Example:

```rust
#[default_task]
fn audit() {
    exec!("cargo", "audit");
}
```

## Macros

### deps

The `deps!` macro registers prerequisite tasks into a run-once cache. The task is required to have been run, exactly once, by the enclosing function, each time `tinyrick`... runs.

Example:

```rust
deps!(test);
```

### exec

The `exec!` macro executes commands. Arguments optional.

Example:

```rust
exec!("cargo", "test");
```

For more complex, dynamically calculated arguments, use the tinyrick `exec` function.

# Cargo.toml

tinyrick uses the cargo `features` capability to isolate task dependencies from application dependencies.

Example:

```toml
[package]
name = "arithmancy"
exclude = [
    "tinyrick.rs",
]

[dependencies]
ctor = { version = "0.6.2", optional = true }
die = "0.2.0"
tinyrick = { version = "0.0.25", optional = true }
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

# ENVIRONMENT VARIABLES

## VERBOSE

Default: (unset)

When `1`, enables additional logging.
