use std::io;
use std::io::{BufRead, BufReader};

pub fn run_advent_day_one() {
    let input_file_path = "input_data/day_one.txt";
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let input_file_reader = BufReader::new(input_file);
    let lines = create_line_stream_from_file(input_file_reader);
    let sum_values = lines.map(|l| calc_calibration_val(&l))
        .sum::<i32>();

    println!("Sum of calibration values: {}", sum_values);
}

pub fn create_line_stream_from_stdin(buf_reader: BufReader<io::Stdin>) -> impl Iterator<Item = String> {
    buf_reader.lines()
        .map(|l| l.expect("Could not parse line"))
}

pub fn create_line_stream_from_file(buf_reader: BufReader<std::fs::File>) -> impl Iterator<Item = String> {
    buf_reader.lines()
        .map(|l| l.expect("Could not parse line"))
}

pub fn calc_calibration_val(line: &str) -> i32 {
    let mut first_digit: Option<i32> = None;
    let mut last_digit: Option<i32> = None;

    for c in line.chars() {
        if !c.is_digit(10) {
            continue;
        }

        let digit = c.to_digit(10).unwrap() as i32;

        if first_digit.is_none() {
            first_digit = Some(digit * 10);
        }

        last_digit = Some(digit);
    }

    if first_digit.is_none() || last_digit.is_none() {
        panic!("Invalid input: {}", line);
    }

    return first_digit.unwrap() + last_digit.unwrap();
}
