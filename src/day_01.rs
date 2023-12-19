use std::io;
use std::io::{BufRead, BufReader};

pub fn run_advent_day_one() {
    let input_file_path = "input_data/day_one.txt";
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let input_file_reader = BufReader::new(input_file);
    let lines = create_line_stream_from_file(input_file_reader);
    let sum_values = lines.map(|l| calc_calibration_val(&l)).sum::<i32>();

    println!("Sum of calibration values: {}", sum_values);
}

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

pub fn slice_at_index_matches(base: &str, index: u32, target: &str) -> bool {
    if index as usize + target.len() > base.len() {
        return false;
    }

    return &base[(index as usize)..((index as usize) + target.len())] == target;
}

pub fn convert_to_digit_sequence(line: &str) -> Vec<i32> {
    let word_num_pairs: Vec<(&str, i32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut digits: Vec<i32> = vec![];
    let line_bytes = line.as_bytes();

    'outer: for i in 0..line_bytes.len() {
        if line_bytes[i].is_ascii_digit() {
            digits.push(line_bytes[i] as i32 - b'0' as i32);
            continue;
        }

        for (word, num) in &word_num_pairs {
            if slice_at_index_matches(line, i as u32, word) {
                digits.push(*num);
                continue 'outer;
            }
        }
    }

    return digits;
}

pub fn calc_calibration_val(line: &str) -> i32 {
    let mut first_digit: Option<i32> = None;
    let mut last_digit: Option<i32> = None;

    let digits = convert_to_digit_sequence(line);

    if digits.len() > 0 {
        first_digit = Some(digits[0]);
        last_digit = Some(digits[digits.len() - 1]);
    }

    if first_digit.is_none() || last_digit.is_none() {
        panic!("Invalid input: {}", line);
    }

    let first_val = first_digit.unwrap() * 10;
    let last_val = last_digit.unwrap();

    return first_val + last_val;
}
