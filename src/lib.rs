//! Common build patterns

/// Per-project binary name
pub static BINARY : &str = "tinyrick";

/// Cargo toggle
pub static FEATURE : &str = "letmeout";

/// Declare a dependency on a task that may panic
pub fn deps(task: fn()) {
  task();
}
