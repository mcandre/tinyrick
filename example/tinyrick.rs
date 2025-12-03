//! Build configuration

use tinyrick::*;

/// Build: Lint, test, and then build binaries
#[default_task]
fn build() {
    deps!(build_debug);
    deps!(build_release);
}

/// Security audit
#[task]
fn audit() {
    exec!("cargo", "audit");
}

/// Lint, test, and build debug binaries
#[task]
fn build_debug() {
    deps!(test);
    exec!("cargo", "build");
}

/// Lint, test, and build release binaries
#[task]
fn build_release() {
    deps!(test);
    exec!("cargo", "build", "--release");
}

/// Run cargo check
#[task]
fn cargo_check() {
    exec!("cargo", "check");
}

/// Clean workspaces
#[task]
fn clean() {
    deps!(clean_cargo);
}

/// Run cargo clean
#[task]
fn clean_cargo() {
    exec!("cargo", "clean");
}

/// Run clippy
#[task]
fn clippy() {
    exec!("cargo", "clippy");
}

/// Generate documentation
#[task]
fn doc() {
    exec!("cargo", "doc");
}

/// Static code validation
#[task]
fn lint() {
    deps!(doc);
    deps!(clippy);
}

/// Lint, and then run integration tests
#[task]
fn integration_test() {
    deps!(install);
    exec!("add_two", "-n", "2");
}

/// Lint, and then install artifacts
#[task]
fn install() {
    deps!(lint);
    exec!("cargo", "install", "--force", "--path", ".");
}

/// Publish to crate repository
#[task]
fn publish() {
    exec!("cargo", "publish");
}

/// Run test suites
#[task]
fn test() {
    deps!(unit_test);
    deps!(integration_test);
}

/// Uninstall artifacts
#[task]
fn uninstall() {
    exec!("cargo", "uninstall");
}

/// Lint, then run unit tests
#[task]
fn unit_test() {
    deps!(lint);
    exec!("cargo", "test");
}

/// CLI entrypoint
fn main() {
    run()
}
