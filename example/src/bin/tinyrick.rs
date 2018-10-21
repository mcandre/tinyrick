//! Build configuration

extern crate tinyrick;

use std::env;

/// Run clippy
fn clippy() {
  tinyrick::shell!("cargo", "clippy");
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  tinyrick::shell!("cargo", "build");
}

/// Generate documentation
fn doc() {
  tinyrick::shell!("cargo", "doc");
}

/// Install project
fn install() {
  tinyrick::shell!("cargo", "install", "--force", "--path", ".");
}

/// Uninstall project
fn uninstall() {
  tinyrick::shell!("cargo", "uninstall", env!("CARGO_PKG_NAME"));
}

/// Run unit tests
fn unit_test() {
  tinyrick::shell!("cargo", "test");
}

/// Run integration tests
fn integration_test() {
  tinyrick::deps(build);
  assert!(tinyrick::shell_stdout_utf8!("add_two", "-n", "2") == "4\n");
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Show banner
fn banner() {
  tinyrick::shell!("add_two", "-v");
}

/// Publish to crate repository
fn publish() {
  tinyrick::shell!("cargo", "publish");
}

pub fn main() {
  let args : Vec<String> = env::args()
    .collect();

  let task_names : Vec<&str> = args
    .iter()
    .skip(1)
    .map(String::as_str)
    .collect();

  if task_names.len() == 0 {
    test();
  } else {
    for task_name in task_names {
      match task_name {
        "clippy" => clippy(),
        "lint" => lint(),
        "build" => build(),
        "doc" => doc(),
        "install" => install(),
        "uninstall" => uninstall(),
        "unit_test" => unit_test(),
        "integration_test" => integration_test(),
        "test" => test(),
        "banner" => banner(),
        "publish" => publish(),
        _ => panic!("Unknown task {}", task_name)
      }
    }
  }
}
