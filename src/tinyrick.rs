//! CLI tinyrick tool

extern crate tinyrick;

use std::env::args;
use std::path;

/// CLI entrypoint
fn main() {
  let arguments : Vec<String> = args().collect();

  let tasks : Vec<&str> = arguments
    .iter()
    .skip(1)
    .map(String::as_str)
    .collect();

    tinyrick::exec!(
      "cargo",
      &[
        "build",
        "--bin", tinyrick::BINARY,
        "--features", tinyrick::FEATURE
      ]
    );

  let suffix : &str = if cfg!(windows) {
    ".exe"
  } else {
    ""
  };

  let rick_binary : String = tinyrick::BINARY.to_string() + suffix;

  let target_path : &path::Path = path::Path::new("target");

  let rick_pathbuf : path::PathBuf = target_path
    .join("debug")
    .join(rick_binary);

  let rick_path : &str = rick_pathbuf
    .to_str()
    .unwrap();

  tinyrick::exec!(rick_path, tasks);
}
