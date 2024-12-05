use std::cmp::Ordering;
use std::io::BufReader;

use crate::utils::{create_line_stream_from_file, GuardGremlin};

#[allow(dead_code)]
pub fn run_advent_day_two() {
    let input_file_path = "input_data/day_02.txt";
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let input_file_reader = BufReader::new(input_file);
    let lines = create_line_stream_from_file(input_file_reader);

    let scanner = ReportScanner::new();

    let mut safe_count = 0u32;

    for line in lines {
        let is_safe = scanner.check_safe(line.as_str());
        if is_safe {
            safe_count += 1;
        }
    }

    println!("Safe count {safe_count}");
}

#[derive(PartialEq, Eq)]
enum Mode {
    Unknown,
    Static,
    Inc,
    Dec,
}

trait ScanReport {
    fn check_safe(&self, line: &str) -> bool;
}

struct ReportScanner {}

impl ReportScanner {
    fn new() -> Self {
        ReportScanner {}
    }
}

impl ScanReport for ReportScanner {
    fn check_safe(&self, s: &str) -> bool {
        const MIN_DELTA: i32 = 1;
        const MAX_DELTA: i32 = 3;

        let mut words = s.split_whitespace();

        let get_val_from_word = |w_opt: Option<&str>| -> Option<i32> {
            match w_opt {
                Some(w) => match w.parse::<i32>() {
                    Ok(v) => Some(v),
                    _ => None,
                },
                _ => None,
            }
        };

        let first_word = words.next();
        let mut mode = Mode::Unknown;
        let mut prev_val: i32 = match get_val_from_word(first_word) {
            Some(v) => v,
            _ => return false,
        };

        for word in words {
            let val: i32 = match get_val_from_word(Some(word)) {
                Some(v) => v,
                _ => return false,
            };
            let diff = prev_val - val;
            let abs_diff = diff.abs();

            // ensure 'prev_val' is updated each interation
            let _gremlin = GuardGremlin::new(|| prev_val = val);

            if !(MIN_DELTA..=MAX_DELTA).contains(&abs_diff) {
                return false;
            }

            let new_mode = match diff.cmp(&0i32) {
                Ordering::Greater => Mode::Inc,
                Ordering::Less => Mode::Dec,
                Ordering::Equal => Mode::Static,
            };

            if mode == Mode::Unknown {
                mode = new_mode;
                continue;
            }

            if new_mode != mode {
                return false;
            }
        }

        true
    }
}
