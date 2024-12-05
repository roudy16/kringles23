use std::collections::HashMap;
use std::io::BufReader;

use crate::utils::create_line_stream_from_file;

#[allow(dead_code)]
pub fn run_advent_day_one() {
    let input_file_path = "input_data/day_01.txt";
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let input_file_reader = BufReader::new(input_file);
    let lines = create_line_stream_from_file(input_file_reader);

    // get sorted lists from input
    let num_pairs: Vec<(i64, i64)> = lines
        .map(|l| create_number_pair(l.as_str()).unwrap())
        .collect();
    let (mut first_list, mut second_list): (Vec<i64>, Vec<i64>) = num_pairs.iter().cloned().unzip();
    first_list.sort();
    second_list.sort();

    // get the 'total distance' (part 1)
    let total_dist = calc_list_diff(&first_list, &second_list);

    // compute 'similarity score' (part 2)
    let second_list_occurrences = calc_occurrences(&second_list);
    let similarity_score = calc_similarity(&first_list, &second_list_occurrences);

    println!("Total dist: {:?}", total_dist);
    println!("Similarity score: {:?}", similarity_score);
}

pub fn calc_list_diff(lhs: &[i64], rhs: &[i64]) -> i64 {
    lhs.iter()
        .cloned()
        .zip(rhs.iter().cloned())
        .map(|(a, b)| (a - b).abs())
        .reduce(|acc, v| acc + v)
        .unwrap()
}

pub fn calc_occurrences(nums: &[i64]) -> HashMap<i64, i64> {
    let mut counts = HashMap::<i64, i64>::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
}

pub fn calc_similarity(l: &[i64], occurances: &HashMap<i64, i64>) -> i64 {
    let mut acc: i64 = 0;
    for &num in l {
        if occurances.contains_key(&num) {
            acc += occurances.get(&num).unwrap() * num;
        }
    }
    acc
}

pub fn create_number_pair(s: &str) -> Result<(i64, i64), String> {
    let mut words = s.split_whitespace();
    let maybe_word0 = words.next();
    let maybe_word1 = words.next();

    if maybe_word0.is_none() {
        return Err(String::from("expected two words on line, got zero"));
    }

    if maybe_word1.is_none() {
        return Err(String::from("expected two words on line, got one"));
    }

    if words.count() != 0 {
        return Err(String::from("too many words on line"));
    }

    let word0 = maybe_word0.unwrap();
    let word1 = maybe_word1.unwrap();
    let maybe_num0 = word0.parse::<i64>();
    let maybe_num1 = word1.parse::<i64>();

    if maybe_num0.is_err() {
        return Err(format!("failed to parse word as number: '{word0}'"));
    }
    if maybe_num1.is_err() {
        return Err(format!("failed to parse word as number: '{word1}'"));
    }

    let num0 = maybe_num0.unwrap();
    let num1 = maybe_num1.unwrap();

    Ok((num0, num1))
}
