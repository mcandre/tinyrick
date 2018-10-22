//! CLI tinyrick tool

// #[macro_use]
extern crate tinyrick;

use std::env;
use std::path;

/// CLI entrypoint
fn main() {
  let args : Vec<String> = env::args().collect();

  let tasks : Vec<&str> = args
    .iter()
    .skip(1)
    .map(String::as_str)
    .collect();

    tinyrick::shell!(
      "cargo",
      &[
        "build",
        "--bin", tinyrick::BINARY,
        "--features", tinyrick::FEATURE
      ]
    );

  let rick_binary : &str = match cfg!(windows) {
    true => "rick.exe",
    _ => "rick"
  };

  let target_path : &path::Path = path::Path::new("target");

  let rick_pathbuf : path::PathBuf = target_path
    .join("debug")
    .join(rick_binary);

  let rick_path : &str = rick_pathbuf
    .to_str()
    .unwrap();

  tinyrick::shell!(rick_path, tasks);
}
