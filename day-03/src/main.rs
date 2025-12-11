// Advent of Code 2025: Day 3
// https://adventofcode.com/2025/day/3
// Usage: `cargo run <input-file>

use std::{env, fs, str};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let banks: Vec<&str> = input.trim().split("\n").collect();

    let part_1: usize = banks
        .iter()
        .map(|bank| highest_joltage(&bank.as_bytes(), 2))
        .sum();

    let part_2: usize = banks
        .iter()
        .map(|bank| highest_joltage(&bank.as_bytes(), 12))
        .sum();

    println!("The total joltage with 2 batteries is {}", part_1);
    println!("The total joltage with 12 batteries is {}", part_2);
}

fn highest_joltage(bank: &[u8], num_batteries: usize) -> usize {
    let (at, val) = first_max(&bank[0..=bank.len() - num_batteries]);
    if num_batteries == 1 {
        val
    } else {
        val * 10usize.pow(num_batteries as u32 - 1)
            + highest_joltage(&bank[at + 1..], num_batteries - 1)
    }
}

fn first_max(bank: &[u8]) -> (usize, usize) {
    let max_val = bank.iter().max().unwrap();
    let at = bank.iter().position(|val| val == max_val).unwrap();
    (at, (max_val - '0' as u8).into())
}
