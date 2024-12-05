use std::io;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn create_line_stream_from_stdin(
    buf_reader: BufReader<io::Stdin>,
) -> impl Iterator<Item = String> {
    buf_reader.lines().map(|l| l.expect("Could not parse line"))
}

#[allow(dead_code)]
pub fn create_line_stream_from_file(
    buf_reader: BufReader<std::fs::File>,
) -> impl Iterator<Item = String> {
    buf_reader.lines().map(|l| l.expect("Could not parse line"))
}

pub struct GuardGremlin<T: FnOnce()> {
    cb: Option<T>,
}

impl<T: FnOnce()> GuardGremlin<T> {
    pub fn new(cb: T) -> Self {
        Self { cb: Some(cb) }
    }
}

impl<T: FnOnce()> Drop for GuardGremlin<T> {
    fn drop(&mut self) {
        if let Some(cb) = self.cb.take() {
            cb();
        }
    }
}
