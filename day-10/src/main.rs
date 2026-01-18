// Advent of Code 2025: Day 10
// https://adventofcode.com/2025/day/10
// Usage: `cargo run <input-file>

use day_10::linear_algebra::{
    extract_parametric_solution, extract_pivots, gauss_jordan_to_rref, naive_max_bounds,
    AffineExpression,
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
        let global_max_bounds = naive_max_bounds(&machine.equations.clone());
        let rows = equations.len();
        let cols = equations[0].len();
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
                let var = pivot_data.free_columns[0];
                let naive_max = global_max_bounds[&var].try_into().unwrap();
                let (min, max) = free_variable_bounds(&parametric_solution, var, 0, naive_max);
                let sum_equation: AffineExpression = parametric_solution.clone().into_iter().sum();

                let mut values = HashMap::from([(var, Rational::from(min as isize))]);
                let mut minimal_sum = Rational::from(isize::MAX);

                for val in min..=max {
                    values.insert(var, Rational::from(val as isize));
                    let all_positive_integers = parametric_solution
                        .iter()
                        .all(|ae| ae.eval(&values).is_non_negative_integer());
                    if !all_positive_integers {
                        continue;
                    }
                    let sum = sum_equation.eval(&values);
                    if !sum.is_non_negative_integer() {
                        continue;
                    }
                    if sum < minimal_sum {
                        minimal_sum = sum;
                    }
                }

                minimal_sum
            }
            2 => {
                let solution = parametric_solution.clone();
                let sum_equation: AffineExpression = parametric_solution.into_iter().sum();

                let mut bounds: HashMap<usize, (usize, usize)> = HashMap::new();

                // get the bounds we can derive from the current solution
                for var in pivot_data.free_columns {
                    bounds.insert(
                        var,
                        free_variable_bounds(
                            &solution,
                            var,
                            0,
                            global_max_bounds[&var].try_into().unwrap(),
                        ),
                    );
                }

                // pick our outer loop
                let (outer, (outer_min, outer_max)) =
                    bounds.iter().min_by_key(|(_, (_, max))| max).unwrap();

                let (inner, (inner_min, inner_max)) =
                    bounds.iter().find(|(var, _)| var != &outer).unwrap();

                let mut minimal_sum = Rational::from(isize::MAX);

                for outer_val in *outer_min..=*outer_max {
                    let mut values: HashMap<usize, Rational> = HashMap::new();
                    values.insert(*outer, (outer_val as isize).into());
                    for inner_val in *inner_min..=*inner_max {
                        values.insert(*inner, (inner_val as isize).into());

                        let all_positive_integers = solution
                            .iter()
                            .all(|ae| ae.eval(&values).is_non_negative_integer());
                        if !all_positive_integers {
                            continue;
                        }
                        let sum = sum_equation.eval(&values);
                        if !sum.is_non_negative_integer() {
                            continue;
                        }
                        if sum < minimal_sum {
                            minimal_sum = sum;
                        }
                    }
                }

                minimal_sum
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

// derive min/max bounds from the parametric solution for the given free variable
// if there are no affine expressions that only contain the one free variable,
// no bounds will be returned other than the defaults
fn free_variable_bounds(
    parametric_solution: &Vec<AffineExpression>,
    free_variable: usize,
    min: isize,
    max: isize,
) -> (usize, usize) {
    // theory: there will be no equation where there is no upper bound found here
    // we should assert this and if needed we can derive an upper bound from the original equations
    let mut min: isize = min as isize;
    let mut max: isize = max as isize;

    for affine_expression in parametric_solution {
        // TODO: for more precise bounds reduce to single variable by partially evaling
        // with min max bounds for the other free variables in the expression if any
        if affine_expression.free_variable_coefficients.len() > 1 {
            continue;
        }
        if let Some(&coefficient) = affine_expression
            .free_variable_coefficients
            .get(&free_variable)
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
