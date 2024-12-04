use std::io;
use std::io::{BufRead, BufReader};

pub fn create_line_stream_from_stdin(
    buf_reader: BufReader<io::Stdin>,
) -> impl Iterator<Item = String> {
    buf_reader.lines().map(|l| l.expect("Could not parse line"))
}

pub fn create_line_stream_from_file(
    buf_reader: BufReader<std::fs::File>,
) -> impl Iterator<Item = String> {
    buf_reader.lines().map(|l| l.expect("Could not parse line"))
}
