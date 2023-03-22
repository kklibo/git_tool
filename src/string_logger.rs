//! Use `log` to log to a string.

use log::{Log, Metadata, Record};
use std::sync::Mutex;

pub struct StringLogger {
    s: Mutex<String>,
}
impl StringLogger {
    pub const fn new() -> Self {
        Self {
            s: Mutex::new(String::new()),
        }
    }
    pub fn get(&self) -> String {
        let a = self.s.lock().unwrap();
        a.clone()
    }
}
impl Log for StringLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let mut a = self.s.lock().unwrap();
        a.push_str(&format!("{} - {}\n", record.level(), record.args()));
    }

    fn flush(&self) {}
}
