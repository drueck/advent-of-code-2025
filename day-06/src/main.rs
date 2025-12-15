// Advent of Code 2025: Day 6
// https://adventofcode.com/2025/day/6
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    println!("The answer to part 1 is: {}", normal_math(&input));
    println!("The answer to part 2 is: {}", cephalopod_math(&input));
}

fn normal_math(input: &str) -> usize {
    let maths: Vec<Vec<&str>> = input
        .trim()
        .split('\n')
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let operator_index = maths.len() - 1;

    (0..maths[0].len())
        .map(|col| {
            let numbers = (0..operator_index).map(|row| maths[row][col].parse::<usize>().unwrap());
            match maths[operator_index][col] {
                "*" => numbers.product::<usize>(),
                _ => numbers.sum(),
            }
        })
        .sum()
}

fn cephalopod_math(input: &str) -> usize {
    let lines: Vec<&[u8]> = input
        .trim_matches('\n')
        .split('\n')
        .map(|line| line.as_bytes())
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut total: usize = 0;
    let mut col = cols;
    let mut numbers = vec![];

    loop {
        col -= 1;
        let mut row = rows;
        let mut exponent = 0;
        let mut number = 0;
        let mut operator: u8 = 0;

        loop {
            row -= 1;
            let val = lines[row][col];
            match val {
                b'0'..=b'9' => {
                    number += (val - b'0') as usize * 10usize.pow(exponent);
                    exponent += 1;
                }
                b'+' | b'*' => {
                    operator = val;
                }
                _ => {}
            }

            if row == 0 {
                break;
            }
        }

        numbers.push(number);

        match operator {
            b'+' | b'*' => {
                total += if operator == b'+' {
                    numbers.iter().sum::<usize>()
                } else {
                    numbers.iter().product()
                };
                numbers.clear();
                // if we're not at the end, skip the next column
                // because there's an empty one after every operation
                if col != 0 {
                    col -= 1;
                }
            }
            _ => {}
        }

        if col == 0 {
            break;
        }
    }

    total
}
