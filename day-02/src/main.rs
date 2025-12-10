// Advent of Code 2025: Day 2
// https://adventofcode.com/2025/day/2
// Usage: `cargo run <input-file>

use std::{env, fs, ops::RangeInclusive};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let ranges: Vec<RangeInclusive<usize>> = input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("well formed range");
            start.parse::<usize>().expect("valid int")..=end.parse::<usize>().expect("valid int")
        })
        .collect();

    let mut sum_part_1: usize = 0;
    let mut sum_part_2: usize = 0;

    for range in ranges.iter() {
        for invalid in find_invalid_part_1(range.clone()) {
            sum_part_1 += invalid;
        }
    }

    for range in ranges {
        for invalid in find_invalid_part_2(range) {
            sum_part_2 += invalid;
        }
    }

    println!(
        "The sum of the invalid part numbers for part 1 was: {}",
        sum_part_1
    );
    println!(
        "The sum of the invalid part numbers for part 2 was: {}",
        sum_part_2
    );
}

fn find_invalid_part_1(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut invalid = vec![];
    for num in range {
        let num_as_string = num.to_string();
        let len = num_as_string.len();
        if len % 2 == 0 {
            let (first, second) = num_as_string.split_at(len / 2);
            if first == second {
                invalid.push(num)
            }
        }
    }
    return invalid;
}

fn find_invalid_part_2(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut invalid = vec![];
    for num in range {
        let num_as_string = num.to_string();
        let len = num_as_string.len();
        let num_as_bytes = num_as_string.as_bytes();
        for chunk_size in 1..=(len / 2) {
            if len % chunk_size != 0 {
                continue;
            }
            let first = &num_as_bytes[..chunk_size];
            if num_as_bytes
                .chunks_exact(chunk_size)
                .skip(1)
                .all(|chunk| chunk == first)
            {
                invalid.push(num);
                break;
            }
        }
    }
    return invalid;
}
