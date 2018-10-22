//! Common build patterns

/// Per-project binary name
pub static BINARY : &str = "rick";

/// Cargo toggle
pub static FEATURE : &str = "letmeout";

/// Environment name controlling verbosity
pub static VERBOSE_ENVIRONMENT_NAME : &str = "VERBOSE";

/// Declare a dependency on a task that may panic
pub fn deps(task: fn()) {
  task();
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the command object.
#[macro_export]
macro_rules! shell_mut_with_arguments {
  ($p : expr, $a : expr) => {
    {
      use std::env::var;
      use std::process::Command;

      if var(tinyrick::VERBOSE_ENVIRONMENT_NAME).is_ok() {
        println!("{} {}", $p, $a.join(" "));
      }

      Command::new($p)
        .args($a)
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program. Can also accept CLI arguments collection.
/// Returns the command object.
#[macro_export]
macro_rules! shell_mut {
  ($p : expr) => {
    {
      tinyrick::shell_mut_with_arguments!($p, &[])
    }
  };
  ($p : expr, $a : expr) => {
    {
      tinyrick::shell_mut_with_arguments!($p, $a)
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the output object.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell_output {
  ($p : expr) => {
    {
      tinyrick::shell_mut!($p)
        .output()
        .unwrap()
    }
  };
  ($p : expr, $a : expr) => {
    {
      tinyrick::shell_mut!($p, $a)
        .output()
        .unwrap()
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the stdout stream.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell_stdout {
  ($p : expr) => {
    {
      tinyrick::shell_output!($p)
        .stdout
    }
  };
  ($p : expr, $a : expr) => {
    {
      tinyrick::shell_output!($p, $a)
        .stdout
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the stdout stream.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell_stderr {
  ($p : expr) => {
    {
      tinyrick::shell_output!($p)
        .stderr
    }
  };
  ($p : expr, $a : expr) => {
    {
      tinyrick::shell_output!($p, $a)
        .stderr
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the complete stdout string.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell_stdout_utf8 {
  ($p : expr) => {
    {
      String::from_utf8(tinyrick::shell_stdout!($p))
        .unwrap()
    }
  };
  ($p : expr, $a : expr) => {
    {
      String::from_utf8(tinyrick::shell_stdout!($p, $a))
        .unwrap()
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the complete stderr string.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell_stderr_utf8 {
  ($p : expr) => {
    {
      String::from_utf8(tinyrick::shell_stderr!($p))
        .unwrap()
    }
  };
  ($p : expr, $a : expr) => {
    {
      String::from_utf8(tinyrick::shell_stderr!($p, $a))
        .unwrap()
    }
  };
}

/// Hey stupid, avoid shell commands whenever possible!
/// Executes the given program with the given arguments.
/// Returns the status object.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! shell {
  ($p : expr) => {
    {
      tinyrick::shell_mut!($p)
        .status()
        .unwrap()
    }
  };
  ($p : expr, $a : expr) => {
    {
      tinyrick::shell_mut!($p, $a)
        .status()
        .unwrap()
    }
  };
}

/// Register tasks with CLI entrypoint.
/// The first entry is the default task,
/// When no tasks are named in CLI arguments.
#[macro_export]
macro_rules! wubba_lubba_dub_dub {
  ($d : expr ; $($t : expr),*) => {
    fn main() {
      use std::env;

      let args : Vec<String> = env::args()
        .collect();

      let task_names : Vec<&str> = args
        .iter()
        .skip(1)
        .map(String::as_str)
        .collect();

      if task_names.len() == 0 {
        $d();
      } else {
        for task_name in task_names {
          match task_name {
            stringify!($d) => $d(),
            $(stringify!($t) => $t(),)*
            _ => panic!("Unknown task {}", task_name)
          }
        }
      }
    }
  };
}
