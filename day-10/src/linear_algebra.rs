use std::collections::HashMap;

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
    pub pivot_columns: Vec<usize>,
    pub free_columns: Vec<usize>,
    pub pivot_row_for_col: HashMap<usize, usize>,
}

pub fn extract_pivots(rref_matrix: &Vec<Vec<Rational>>) -> PivotData {
    let rows = rref_matrix.len();
    let cols = rref_matrix[0].len();
    let mut pivot_columns = vec![];
    let mut free_columns = vec![];
    let mut pivot_row_for_col: HashMap<usize, usize> = HashMap::new();

    'rows: for row in 0..rows {
        for col in 0..(cols - 1) {
            let val = rref_matrix[row][col];
            if val != 0.into() {
                if val == 1.into() {
                    pivot_columns.push(col);
                    pivot_row_for_col.insert(col, row);
                }
                continue 'rows;
            }
        }
    }

    for col in 0..(cols - 1) {
        if !pivot_columns.contains(&col) {
            free_columns.push(col);
        }
    }

    PivotData {
        pivot_columns,
        free_columns,
        pivot_row_for_col,
    }
}
