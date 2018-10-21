//! Build configuration

extern crate tinyrick;

use std::env;
use std::process::Command;

/// Run clippy
fn clippy() {
  Command::new("cargo")
    .arg("clippy")
    .status()
    .expect("Error running clippy");
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  Command::new("cargo")
    .arg("build")
    .status()
    .expect("Error building project");
}

/// Generate documentation
fn doc() {
  Command::new("cargo")
    .arg("doc")
    .status()
    .expect("Error generating documentation");
}

/// Install project
fn install() {
  Command::new("cargo")
    .args(&["install", "--force", "--path", "."])
    .status()
    .expect("Error installing project");
}

/// Uninstall project
fn uninstall() {
  Command::new("cargo")
    .args(&["uninstall", "arithmancy"])
    .status()
    .expect("Error installing project");
}

/// Run unit tests
fn unit_test() {
  Command::new("cargo")
    .arg("test")
    .status()
    .expect("Error during tests");
}

/// Run integration tests
fn integration_test() {
  tinyrick::deps(build);

  let bin = "add_two";

  let output = Command::new(bin)
    .args(&["-n", "2"])
    .output()
    .expect(&format!("Error running binary {}", bin));

  String::from_utf8(output.stdout)
    .map(|stdout| { assert!(stdout == "4\n") })
    .expect(&format!("Error parsing stdout as UTF-8 on binary {}", bin));
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Show banner
fn banner() {
  Command::new("add_two")
    .arg("-v")
    .status()
    .expect("Error running 'add_two -v'");
}

/// Publish to crate repository
fn publish() {
  Command::new("cargo")
    .arg("publish")
    .status()
    .expect("Error publishing package");
}

pub fn main() {
  let args : Vec<String> = env::args().collect();

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
