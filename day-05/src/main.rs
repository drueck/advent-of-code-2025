// Advent of Code 2025: Day 5
// https://adventofcode.com/2025/day/5
// Usage: `cargo run <input-file>

use std::cmp::{max, min};
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

    let mut merged_ranges = MergedRanges::new();
    for range in fresh_ranges {
        merged_ranges.add(&range);
    }

    let available_fresh = available
        .iter()
        .filter(|ingredient| merged_ranges.contain(&ingredient))
        .count();

    println!("The number of available fresh ingredients is {available_fresh}");

    println!(
        "The total number of fresh ingredients is {}",
        merged_ranges.len()
    );
}

struct MergedRanges {
    ranges: Vec<RangeInclusive<usize>>,
}

impl MergedRanges {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add(&mut self, new_range: &RangeInclusive<usize>) {
        let mut updated_ranges = vec![];
        let mut new_merged = new_range.clone();
        for range in &self.ranges[..] {
            if let Some(merged) = try_merge(&new_merged, &range) {
                new_merged = merged;
            } else {
                updated_ranges.push(range.clone());
            }
        }
        updated_ranges.push(new_merged);
        self.ranges = updated_ranges;
    }

    fn contain(&self, item: &usize) -> bool {
        self.ranges.iter().any(|range| range.contains(item))
    }

    fn len(&self) -> usize {
        self.ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }
}

fn try_merge(
    range_a: &RangeInclusive<usize>,
    range_b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if (range_a.start() <= range_b.end()) & (range_b.start() <= range_a.end()) {
        return Some(min(*range_a.start(), *range_b.start())..=max(*range_a.end(), *range_b.end()));
    }
    None
}
