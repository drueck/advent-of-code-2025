use core::fmt;
use std::collections::HashMap;
use std::iter::Sum;

use crate::rational::Rational;

pub fn gauss_jordan_to_rref(equations: &mut Vec<Vec<Rational>>) {
    let rows = equations.len();
    let cols = equations[0].len();

    let mut pivot_row: usize = 0;
    let mut pivot_col_start: usize = 0;

    loop {
        if pivot_row == equations.len() {
            break;
        }

        if let Some((new_pivot_row, pivot_col)) = find_pivot(equations, pivot_row, pivot_col_start)
        {
            // find pivot
            if new_pivot_row != pivot_row {
                equations.swap(pivot_row, new_pivot_row);
            }
            // normalize
            let p = equations[pivot_row][pivot_col];
            if p != 1.into() {
                for col in 0..cols {
                    equations[pivot_row][col] /= p;
                }
            }
            // eliminate that column everywhere else above and below
            for row in 0..rows {
                if row == pivot_row {
                    continue;
                }

                let factor = equations[row][pivot_col];
                if factor == 0.into() {
                    continue;
                }

                for col in 0..cols {
                    let pivot_row_val = equations[pivot_row][col];
                    equations[row][col] -= factor * pivot_row_val
                }
            }
            pivot_col_start = pivot_col + 1;
            pivot_row += 1;
        } else {
            break;
        }
    }

    // sanity check assertion
    // if we have a row something like [0 0 0 0 | 5] that's invalid/impossible
    for row in 0..rows {
        if (0..(cols - 1)).all(|col| equations[row][col] == 0.into())
            && equations[row][cols - 1] != 0.into()
        {
            panic!("row had all zeros except a non-zero in the rhs which is impossible");
        }
    }
}

fn find_pivot(
    equations: &Vec<Vec<Rational>>,
    pivot_row: usize,
    pivot_col_start: usize,
) -> Option<(usize, usize)> {
    for col in pivot_col_start..equations[0].len() - 1 {
        for row in pivot_row..equations.len() {
            if equations[row][col] == 1.into() {
                return Some((row, col));
            }
        }
        for row in pivot_row..equations.len() {
            if equations[row][col] != 0.into() {
                return Some((row, col));
            }
        }
    }
    None
}

#[derive(Debug)]
pub struct PivotData {
    pub pivot_row_for_col: HashMap<usize, usize>,
    pub free_columns: Vec<usize>,
}

pub fn extract_pivots(rref_matrix: &Vec<Vec<Rational>>) -> PivotData {
    let rows = rref_matrix.len();
    let cols = rref_matrix[0].len();
    let mut free_columns = vec![];
    let mut pivot_row_for_col: HashMap<usize, usize> = HashMap::new();

    'rows: for row in 0..rows {
        for col in 0..(cols - 1) {
            let val = rref_matrix[row][col];
            if val != 0.into() {
                if val == 1.into() {
                    pivot_row_for_col.insert(col, row);
                }
                continue 'rows;
            }
        }
    }

    for col in 0..(cols - 1) {
        if !pivot_row_for_col.contains_key(&col) {
            free_columns.push(col);
        }
    }

    PivotData {
        free_columns,
        pivot_row_for_col,
    }
}

// an expression that defines a variable in terms of a constant
// plus the sum of free variables times their coefficients
#[derive(Debug, Clone)]
pub struct AffineExpression {
    pub dependent_variable: usize,
    pub constant: Rational,
    pub free_variable_coefficients: HashMap<usize, Rational>,
}

impl AffineExpression {
    pub fn eval(&self, values: &HashMap<usize, Rational>) -> Rational {
        let mut result = self.constant;

        assert_eq!(self.free_variable_coefficients.len(), values.len());

        for (var, value) in values.iter() {
            assert!(self.free_variable_coefficients.contains_key(var));
            result += self.free_variable_coefficients[var] * *value
        }

        result
    }
}

impl<'a> Sum for AffineExpression {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let result = AffineExpression {
            dependent_variable: (b's' - b'a') as usize,
            constant: 0.into(),
            free_variable_coefficients: HashMap::new(),
        };

        iter.fold(result, |mut result, ae| {
            result.constant += ae.constant;
            for (i, c) in ae.free_variable_coefficients {
                *result
                    .free_variable_coefficients
                    .entry(i)
                    .or_insert(0.into()) += c;
            }
            result
        })
    }
}

impl fmt::Display for AffineExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dep = var(self.dependent_variable);
        let mut free: Vec<(usize, Rational)> = self
            .free_variable_coefficients
            .iter()
            .map(|(i, c)| (*i, *c))
            .collect();

        free.sort_unstable_by_key(|(i, _)| *i);

        let free_equation = free
            .iter()
            .map(|(i, c)| free_var(*i, *c))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{dep} = {} {free_equation}", self.constant)
    }
}

fn var(index: usize) -> char {
    (b'a' + index as u8) as char
}

fn free_var(index: usize, coefficient: Rational) -> String {
    let sign = if coefficient.numerator < 0 { '-' } else { '+' };
    let coef = coefficient.abs();
    let coef = if coef == 1.into() {
        "".into()
    } else {
        format!("{coef}")
    };
    format!("{} {}{}", sign, coef, var(index))
}

pub fn extract_parametric_solution(
    rref_matrix: &Vec<Vec<Rational>>,
    pivot_data: &PivotData,
) -> Vec<AffineExpression> {
    let rhs_col = rref_matrix[0].len() - 1;

    (0..rhs_col)
        .map(|col| {
            // check if it's a pivot col or free col
            if pivot_data.pivot_row_for_col.contains_key(&col) {
                let row = pivot_data.pivot_row_for_col[&col];
                let constant = rref_matrix[row][rhs_col];
                let free_variable_coefficients = pivot_data
                    .free_columns
                    .iter()
                    .map(|free_col| (*free_col, -rref_matrix[row][*free_col]))
                    .filter(|(_free_col, coefficient)| *coefficient != 0.into())
                    .collect();
                AffineExpression {
                    dependent_variable: col,
                    constant,
                    free_variable_coefficients,
                }
            } else {
                let constant = 0.into();
                let free_variable_coefficients = HashMap::from([(col, 1.into())]);
                AffineExpression {
                    dependent_variable: col,
                    constant,
                    free_variable_coefficients,
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_affine_expressions() {
        let a = AffineExpression {
            dependent_variable: 0,
            constant: 5.into(),
            free_variable_coefficients: HashMap::from([(2, Rational::from(-1))]),
        };
        let b = AffineExpression {
            dependent_variable: 1,
            constant: Rational::from(-7),
            free_variable_coefficients: HashMap::from([
                (2, Rational::from(-7)),
                (3, Rational::from(2)),
            ]),
        };

        let sum: AffineExpression = [a, b].into_iter().sum();

        assert_eq!(format!("{sum}"), String::from("s = -2 - 8c + 2d"));
    }

    #[test]
    fn eval_affine_expression() {
        let a = AffineExpression {
            dependent_variable: 0,
            constant: 5.into(),
            free_variable_coefficients: HashMap::from([(2, Rational::from(-1))]),
        };

        let values = HashMap::from([(2, Rational::from(2))]);

        assert_eq!(a.eval(&values), Rational::from(3));
    }
}
