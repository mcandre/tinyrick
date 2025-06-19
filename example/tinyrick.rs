//! Build configuration

extern crate tinyrick;

/// Security audit
fn audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Run cargo check
fn cargo_check() {
    tinyrick::exec!("cargo", &["check"]);
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

/// Lint, and then install artifacts
fn install() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Uninstall artifacts
fn uninstall() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Lint, and then run unit tests
fn unit_test() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["test"]);
}

/// Lint, and then run integration tests
fn integration_test() {
    tinyrick::deps(install);

    assert!(tinyrick::exec_stdout_utf8!("add_two", &["-n", "2"]) == "4\n");
    assert!(!tinyrick::exec_status!("add_two").success());
}

/// Lint, and then run tests
fn test() {
    tinyrick::deps(unit_test);
    tinyrick::deps(integration_test);
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

/// Lint, test, and then build binaries
fn build() {
    tinyrick::deps(build_debug);
    tinyrick::deps(build_release);
}

/// Show banner
fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// Publish to crate repository
fn publish() {
    tinyrick::exec!("cargo", &["publish"]);
}

/// Run cargo clean
fn clean_cargo() {
    tinyrick::exec!("cargo", &["clean"]);
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_cargo);
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
        cargo_check,
        clippy,
        lint,
        doc,
        install,
        uninstall,
        unit_test,
        integration_test,
        test,
        build_debug,
        build_release,
        build,
        banner,
        publish,
        clean,
        clean_cargo
    );
}
