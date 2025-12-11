// Advent of Code 2025: Day 3
// https://adventofcode.com/2025/day/3
// Usage: `cargo run <input-file>

use std::{env, fs, str};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let banks: Vec<&str> = input.trim().split("\n").collect();

    let total: usize = banks
        .iter()
        .map(|bank| highest_joltage(&bank.as_bytes()))
        .sum();

    println!("The total joltage is {}", total);
}

fn highest_joltage(bank: &[u8]) -> usize {
    let (at, mut digit_1) = first_max(&bank);
    let digit_2;

    if at == bank.len() - 1 {
        digit_2 = digit_1;
        digit_1 = bank[..at].iter().max().unwrap()
    } else {
        digit_2 = bank[at + 1..].iter().max().unwrap();
    };

    ((*digit_1 - '0' as u8) * 10u8 + (*digit_2 - '0' as u8)) as usize
}

fn first_max(bank: &[u8]) -> (usize, &u8) {
    let max_val = bank.iter().max().unwrap();
    let at = bank.iter().position(|val| val == max_val).unwrap();
    (at, max_val)
}
