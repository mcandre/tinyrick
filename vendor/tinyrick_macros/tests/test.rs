extern crate tinyrick_models;

use tinyrick_macros::{default_task, task};

use std::sync;

static X: sync::LazyLock<sync::Mutex<u64>> = sync::LazyLock::new(|| sync::Mutex::new(0));

#[default_task]
fn sit() {}

#[test]
fn test_default_task() {
    assert!(
        &*tinyrick_models::DEFAULT_TASK
            .lock()
            .unwrap()
            .unwrap_or("nop")
            == "sit"
    );
}

#[task]
fn increment() {
    let mut y = X.lock().unwrap();
    *y += 1;
}

#[test]
fn test_task() {
    assert!(*X.lock().unwrap() == 0);
    let t = &tinyrick_models::TASKS.lock().unwrap()["increment"];
    t();
    assert!(*X.lock().unwrap() == 1);
}
