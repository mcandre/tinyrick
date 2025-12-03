//! Common build patterns

extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate tinyrick_macros;
extern crate tinyrick_models;

pub use ctor::ctor;
pub use die::die;
pub use tinyrick_macros::{default_task, task};
pub use tinyrick_models::{DEFAULT_TASK, DEPENDENCY_CACHE, TASKS};

use std::env;
use std::process;

/// Cargo toggle
pub static FEATURE: &str = "letmeout";

/// Environment name controlling verbosity
pub static VERBOSE_ENVIRONMENT_NAME: &str = "VERBOSE";

/// deps registers a task prerequisite.
#[macro_export]
macro_rules! deps {
    ($t : expr) => {
        {
            let mut has_run = false;

            {
                let cache_lock = ::tinyrick::DEPENDENCY_CACHE.lock();
                has_run = match cache_lock {
                    Err(e) => ::die::die!(1; e.to_string()),
                    Ok(cache) => cache.contains_key("$t"),
                };
            }

            if !has_run {
                $t();

                let mut cache_lock = ::tinyrick::DEPENDENCY_CACHE.lock();

                match cache_lock {
                    Err(e) => ::die::die!(1; e.to_string()),
                    Ok(mut cache) => cache.insert("$t", true),
                };
            }
        }
    };
}

/// Show registered tasks
pub fn list_tasks() {
    let d = &match DEFAULT_TASK.lock() {
        Err(e) => die!(1; e.to_string()),
        Ok(d_guard) => d_guard.unwrap_or(""),
    };

    if d != &"" {
        println!("Default Task: {}\n", d);
    }

    let t_guard = match TASKS.lock() {
        Err(e) => die!(1; e.to_string()),
        Ok(guard) => guard,
    };
    let mut ts = t_guard.keys().collect::<Vec<&&str>>();
    ts.sort();

    println!("Tasks:");

    if ts.is_empty() {
        println!("(none)")
    }

    for t in ts {
        println!(" * {}", t);
    }
}

/// Run processes task name(s) from CLI arguments.
pub fn run() {
    let task_string_names = env::args().collect::<Vec<String>>();

    let mut task_names: Vec<&str> = task_string_names
        .iter()
        .skip(1)
        .map(String::as_str)
        .collect();

    if task_names.is_empty() {
        let t_lock = DEFAULT_TASK.lock();

        match t_lock {
            Err(e) => die!(1; e.to_string()),
            Ok(t_guard) => match t_guard.as_ref() {
                Some(t) => task_names.push(t),
                _ => {
                    die!(1; "error: missing default task");
                }
            },
        }
    }

    if task_names.contains(&"-l") || task_names.contains(&"--list") {
        list_tasks();
        die!(0);
    }

    let ts_lock = TASKS.lock();
    let ts = match ts_lock {
        Err(e) => die!(1; e.to_string()),
        Ok(ts_guard) => ts_guard,
    };

    for task_name in task_names {
        match ts.get(task_name) {
            Some(t) => t(),
            _ => die!(1; format!("error: unknown task: {}", task_name)),
        }
    }
}

/// Query common host binary suffix
pub fn binary_suffix() -> String {
    if cfg!(windows) {
        return ".exe".to_string();
    }

    String::new()
}

/// exec runs the given command.
///
/// On error, terminates the current process nonzero.
pub fn exec(command: &str, args: &[&str]) {
    if command.is_empty() {
        die!("error: blank command");
    }

    let command_str = if args.is_empty() {
        command
    } else {
        &format!("{} {}", command, args.join(" "))
    };

    if env::var(VERBOSE_ENVIRONMENT_NAME).is_ok() {
        println!("info: running command: {}", command_str);
    }

    let success = match process::Command::new(command).args(args).status() {
        Err(e) => die!(format!(
            "error: unprocessable command: {}: {}",
            command_str,
            e.to_string()
        )),
        Ok(status) => status.success(),
    };

    if !success {
        die!(1; format!("error: command failed: {}", command_str));
    }
}

/// exec! runs the given command.
///
/// On error, terminates the current process nonzero.
#[macro_export]
macro_rules! exec {
    ($p : expr $(, $a:expr)* $(,)?) => {
        ::tinyrick::exec($p, &[ $( $a, )* ])
    };
}
