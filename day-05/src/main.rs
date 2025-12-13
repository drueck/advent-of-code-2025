// Advent of Code 2025: Day 5
// https://adventofcode.com/2025/day/5
// Usage: `cargo run <input-file>

use std::ops::RangeInclusive;
use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let (fresh_input, available_input) = input.trim().split_once("\n\n").unwrap();

    let fresh_ranges: Vec<RangeInclusive<_>> = fresh_input
        .split("\n")
        .map(|range_str| {
            let (start, end) = range_str.split_once("-").unwrap();
            start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
        })
        .collect();

    let available: Vec<_> = available_input
        .split("\n")
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let fresh = available
        .iter()
        .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
        .count();

    println!("The number of fresh ingredients is {fresh}");
}
