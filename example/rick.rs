//! Build configuration

extern crate tinyrick;

/// Run clippy
fn clippy() {
  tinyrick::exec!("cargo", &["clippy"]);
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  tinyrick::exec!("cargo", &["build"]);
}

/// Generate documentation
fn doc() {
  tinyrick::exec!("cargo", &["doc"]);
}

/// Install applications
fn install_binaries() {
  tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Install artifacts
fn install() {
  tinyrick::deps(install_binaries);
}

/// Uninstall artifacts
fn uninstall() {
  tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run unit tests
fn unit_test() {
  tinyrick::exec!("cargo", &["test"]);
}

/// Run integration tests
fn integration_test() {
  tinyrick::deps(install);

  assert!(tinyrick::exec_stdout_utf8!("add_two", &["-n", "2"]) == "4\n");
  assert!(!tinyrick::exec!("add_two").success());
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Show banner
fn banner() {
  tinyrick::exec!("add_two", &["-v"]);
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

tinyrick::wubba_lubba_dub_dub!(
  test;
  clippy,
  lint,
  build,
  doc,
  install_binaries,
  install,
  uninstall,
  unit_test,
  integration_test,
  banner,
  publish,
  clean_cargo,
  clean
);
