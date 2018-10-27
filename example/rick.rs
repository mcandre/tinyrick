//! Build configuration

#[macro_use]
extern crate tinyrick;

/// Run clippy
fn clippy() {
  exec!("cargo", &["clippy"]);
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  exec!("cargo", &["build"]);
}

/// Generate documentation
fn doc() {
  exec!("cargo", &["doc"]);
}

/// Install project
fn install() {
  exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Uninstall project
fn uninstall() {
  exec!("cargo", &["uninstall"]);
}

/// Run unit tests
fn unit_test() {
  exec!("cargo", &["test"]);
}

/// Run integration tests
fn integration_test() {
  tinyrick::deps(install);

  assert!(exec_stdout_utf8!("add_two", &["-n", "2"]) == "4\n");
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Show banner
fn banner() {
  exec!("add_two", &["-v"]);
}

/// Publish to crate repository
fn publish() {
  exec!("cargo", &["publish"]);
}

/// Run cargo clean
fn clean_cargo() {
  exec!("cargo", &["clean"]);
}

/// Clean workspaces
fn clean() {
  tinyrick::deps(clean_cargo);
}

wubba_lubba_dub_dub!(
  test;
  clippy,
  lint,
  build,
  doc,
  install,
  uninstall,
  unit_test,
  integration_test,
  banner,
  publish,
  clean_cargo,
  clean
);
