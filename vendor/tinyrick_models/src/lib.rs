//! Data models for tasks

use std::collections::HashMap;
use std::sync;

/// Task models a development operation.
pub type Task = Box<dyn Fn() + Send + Sync>;

/// DEFAULT_TASK registers the default task name.
pub static DEFAULT_TASK: sync::LazyLock<sync::Mutex<Option<&str>>> =
    sync::LazyLock::new(|| sync::Mutex::new(None));

#[test]
fn test_default_task() {
    let task_name = "increment";

    {
        let mut default_task_initial = DEFAULT_TASK.lock().unwrap();
        assert!(default_task_initial.is_none());
        *default_task_initial = Some(task_name);
    }

    let task_guard = DEFAULT_TASK.lock().unwrap();
    let task_option = task_guard.as_ref();
    let task = task_option.unwrap_or(&"nop");
    assert!(task == &"increment");
}

/// TASKS registers tasks by name.
pub static TASKS: sync::LazyLock<sync::Mutex<HashMap<&str, Task>>> =
    sync::LazyLock::new(|| sync::Mutex::new(HashMap::new()));

#[test]
fn test_tasks() {
    let x = sync::Arc::new(sync::Mutex::new(0));
    let x_clone = sync::Arc::clone(&x);
    let x_clone2 = sync::Arc::clone(&x);
    let name = "increment";

    {
        let mut tasks_initial = TASKS.lock().unwrap();
        assert!(tasks_initial.is_empty());
        let task: Task = Box::new(move || {
            let mut x = x_clone.lock().unwrap();
            *x += 1;
        });
        tasks_initial.insert(name, task);
    }

    assert!(*x_clone2.lock().unwrap() == 0);
    let task = &TASKS.lock().unwrap()[name];
    task();
    assert!(*x.lock().unwrap() == 1);
}

/// DependencyCache is a table of completed task prerequisites.
pub type DependencyCache<'a> = HashMap<&'a str, bool>;

/// DEPENDENCY_CACHE registers task prerequisites.
pub static DEPENDENCY_CACHE: sync::LazyLock<sync::Mutex<DependencyCache>> =
    sync::LazyLock::new(|| sync::Mutex::new(HashMap::new()));

#[test]
fn test_dependency_cache() {
    let name = "build";

    {
        let mut cache_initial = DEPENDENCY_CACHE.lock().unwrap();
        assert!(cache_initial.is_empty());
        cache_initial.insert(name, true);
    }

    let build_completed: bool = DEPENDENCY_CACHE.lock().unwrap()[name];
    assert!(build_completed);
}
