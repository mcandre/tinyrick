//! Cargo subcommand

extern crate tinyrick;

use std::env;
use std::process::Command;

/// CLI entrypoint
fn main() {
  let args : Vec<String> = env::args().collect();

  let tasks : Vec<&str> = args
    .iter()
    .skip(2)
    .map(String::as_str)
    .collect();

  let cargo_args : Vec<&str> = [
    vec!("run", "--bin", tinyrick::BINARY, "--features", tinyrick::FEATURE),
    tasks
  ].concat();

  Command::new("cargo")
    .args(cargo_args)
    .status()
    .expect("Error with your tiny rick");
}
