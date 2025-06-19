//! Build configuration

extern crate tinyrick;

/// Security audit
fn audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Show banner
fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// Lint, test, and then build binaries
fn build() {
    tinyrick::deps(build_debug);
    tinyrick::deps(build_release);
}

/// Lint, test, and build debug binaries
fn build_debug() {
    tinyrick::deps(test);
    tinyrick::exec!("cargo", &["build"]);
}

/// Lint, test, and build release binaries
fn build_release() {
    tinyrick::deps(test);
    tinyrick::exec!("cargo", &["build", "--release"]);
}

/// Run cargo check
fn cargo_check() {
    tinyrick::exec!("cargo", &["check"]);
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_cargo);
}

/// Run cargo clean
fn clean_cargo() {
    tinyrick::exec!("cargo", &["clean"]);
}

/// Run clippy
fn clippy() {
    tinyrick::exec!("cargo", &["clippy"]);
}

/// Generate documentation
fn doc() {
    tinyrick::exec!("cargo", &["doc"]);
}

/// Static code validation
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
}

/// Lint, and then run integration tests
fn integration_test() {
    tinyrick::deps(install);

    assert!(tinyrick::exec_stdout_utf8!("add_two", &["-n", "2"]) == "4\n");
    assert!(!tinyrick::exec_status!("add_two").success());
}

/// Lint, and then install artifacts
fn install() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Publish to crate repository
fn publish() {
    tinyrick::exec!("cargo", &["publish"]);
}

/// Run test suites
fn test() {
    tinyrick::deps(unit_test);
    tinyrick::deps(integration_test);
}

/// Uninstall artifacts
fn uninstall() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Lint, then run unit tests
fn unit_test() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["test"]);
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(
        uninstall,
        clean_cargo,
        clean
    );

    tinyrick::wubba_lubba_dub_dub!(
        build;
        audit,
        banner,
        build,
        build_debug,
        build_release,
        cargo_check,
        clean,
        clean_cargo,
        clippy,
        doc,
        install,
        integration_test,
        lint,
        publish,
        test,
        uninstall,
        unit_test
    );
}
