// Advent of Code 2025: Day 10
// https://adventofcode.com/2025/day/10
// Usage: `cargo run <input-file>

use day_10::linear_algebra::{
    extract_parametric_solution, extract_pivots, gauss_jordan_to_rref, AffineExpression, PivotData,
};
use day_10::rational::Rational;
use regex::Regex;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::sync::LazyLock;
use std::{env, fs, str::FromStr};

static MACHINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(.+)\] (.+) \{(.+)\}").unwrap());

static BUTTON_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(([\d,]+)\)").unwrap());

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let machines: Vec<Machine> = input
        .trim()
        .split('\n')
        .map(|line| line.parse().expect("properly formed input"))
        .collect();

    let part_1: usize = machines
        .iter()
        .map(|machine| fewest_presses(&machine))
        .sum();

    let mut part_2: Rational = 0.into();
    let mut part_2_n_machines = 0;

    for machine in &machines[..] {
        let mut equations = machine.equations.clone();
        gauss_jordan_to_rref(&mut equations);
        let pivot_data = extract_pivots(&equations);
        let num_free_vars = pivot_data.free_columns.len();

        let parametric_solution = extract_parametric_solution(&equations, &pivot_data);

        let min_buttons: Rational = match num_free_vars {
            0 => parametric_solution
                .iter()
                .map(|ae| ae.constant)
                .sum::<Rational>(),
            1 => {
                // for 1 free variable:
                // if A > 0, free var lower bound is >= ceil(-C/A)
                // if A < 0, free var upper bound is <= floor(-C/A)
                let (min, max) = free_variable_bounds(&parametric_solution, &pivot_data);
                let sum_equation: AffineExpression = parametric_solution.into_iter().sum();

                let var = pivot_data.free_columns[0];
                let mut values = HashMap::from([(var, Rational::from(min as isize))]);
                let mut minimal_sum = Rational::from(isize::MAX);

                for val in min..=max {
                    values.insert(var, Rational::from(val as isize));
                    let sum = sum_equation.eval(&values);
                    if sum.denominator != 1 {
                        continue;
                    }
                    if sum < minimal_sum {
                        minimal_sum = sum;
                    }
                }

                minimal_sum
            }
            2 => {
                // println!("START");
                // parametric_solution.iter().for_each(|ae| println!("{ae}"));
                0.into()
            }
            _ => 0.into(),
        };

        if min_buttons != 0.into() {
            assert_eq!(min_buttons.denominator, 1);
            part_2_n_machines += 1;
            part_2 += min_buttons;
        }
    }

    println!("part 1: {part_1}");

    println!(
        "partial solution for part 2 with {} machines solved: {}",
        part_2_n_machines, part_2
    );
}

// only implemented for 1 free variable currently
fn free_variable_bounds(
    // machine: &Machine,
    parametric_solution: &Vec<AffineExpression>,
    pivot_data: &PivotData,
) -> (usize, usize) {
    // theory: there will be no equation where there is no upper bound found here
    // we should assert this and if needed we can derive an upper bound from the original equations
    let mut min: isize = 0;
    let mut max: isize = isize::MAX;

    assert_eq!(pivot_data.free_columns.len(), 1);
    let free_column = pivot_data.free_columns[0];

    for affine_expression in parametric_solution {
        if let Some(&coefficient) = affine_expression
            .free_variable_coefficients
            .get(&free_column)
        {
            // if A > 0, free var lower bound is >= ceil(-C/A)
            // if A < 0, free var upper bound is <= floor(-C/A)
            if coefficient > Rational::from(0) {
                let new_min = (-affine_expression.constant / coefficient).ceil();
                if new_min > min {
                    min = new_min;
                }
            } else if coefficient < 0.into() {
                let new_max = (-affine_expression.constant / coefficient).floor();
                if new_max < max {
                    max = new_max;
                }
            }
        }
    }

    assert!(max < isize::MAX);

    (min as usize, max as usize)
}

fn fewest_presses(machine: &Machine) -> usize {
    let mut states: BinaryHeap<State> = BinaryHeap::new();

    // we might be able to get away with just storing the number of button presses?
    // but we might need the actual buttons too now or for part two? TBD
    // maps lights on to (n buttons pressed, list of buttons pressed in order)
    let mut best_paths: HashMap<u32, Vec<u8>> = HashMap::new();

    let initial_state = State {
        lights: 0,
        buttons_pressed: vec![],
    };

    states.push(initial_state);
    best_paths.insert(0, vec![]);

    while let Some(state) = states.pop() {
        for i in 0..machine.buttons_bitmaps.len() {
            let lights = state.lights ^ machine.buttons_bitmaps[i];

            let mut buttons_pressed = state.buttons_pressed.clone();
            buttons_pressed.push(i as u8);

            let new_state = State {
                lights,
                buttons_pressed,
            };

            if new_state.lights == machine.lights_bitmaps {
                return new_state.buttons_pressed.len();
            }

            match best_paths.entry(lights) {
                Entry::Occupied(mut entry) => {
                    if new_state.buttons_pressed.len() < entry.get().len() {
                        // this is better than what we had before, so replace entry
                        // and push on to the heap
                        entry.insert(new_state.buttons_pressed.clone());
                        states.push(new_state);
                    }
                }
                Entry::Vacant(entry) => {
                    // marking as best and pushing onto heap because it's the best we have so far
                    entry.insert(new_state.buttons_pressed.clone());
                    states.push(new_state);
                }
            }
        }
    }

    usize::MAX
}

#[derive(Debug)]
struct Machine {
    lights_bitmaps: u32,       // bitmap with low bit representing the 0th light
    buttons_bitmaps: Vec<u32>, // bitmaps with which lights each button toggles
    joltages: Vec<u32>,
    equations: Vec<Vec<Rational>>, // matrix representing the constraint equations
}

#[derive(Debug)]
struct ParseMachineError;

impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = MACHINE_REGEX.captures(&s).unwrap();

        let lights_bitmaps = captures[1]
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, char)| match char {
                b'#' => 1 << i,
                _ => 0,
            })
            .sum();

        let buttons: Vec<Vec<u32>> = BUTTON_REGEX
            .captures_iter(&captures[2])
            .map(|captures| captures.get(1).unwrap().as_str())
            .map(|button| {
                button
                    .split(',')
                    .map(|light_index| light_index.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let buttons_bitmaps = buttons
            .iter()
            .map(|button| button.iter().map(|light_index| 1 << light_index).sum())
            .collect();

        let joltages: Vec<u32> = captures[3].split(',').map(|s| s.parse().unwrap()).collect();

        let mut equations = vec![vec![Rational::from(0); buttons.len() + 1]; joltages.len()];

        for (button_index, button) in buttons.iter().enumerate() {
            for joltage_index in button.iter() {
                equations[*joltage_index as usize][button_index] = 1.into();
            }
        }

        let ji = buttons.len();
        for (i, joltage) in joltages.iter().enumerate() {
            equations[i][ji] = Rational::from(*joltage as isize);
        }

        Ok(Self {
            lights_bitmaps,
            buttons_bitmaps,
            joltages,
            equations,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    buttons_pressed: Vec<u8>,
    lights: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.buttons_pressed.len().cmp(&self.buttons_pressed.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
