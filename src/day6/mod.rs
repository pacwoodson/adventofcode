use std::fmt::Display;
use crate::utils;

pub fn solve(input: &String, _: bool) -> (String, String) {
    let lines = utils::lines_to_vec(input);
    let (data_lines, ops_line) = lines.split_at(lines.len() - 1);
    
    // Parse operators
    let ops: Vec<char> = ops_line[0]
        .split_whitespace()
        .filter_map(|s| match s {
            "+" => Some('+'),
            "*" => Some('*'),
            _ => None,
        })
        .collect();

    // Part 1: Parse as numbers and transpose
    let data: Vec<Vec<u64>> = data_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        })
        .collect();

    let n_operations = data.first().map(|r| r.len()).unwrap_or(0);
    let transposed: Vec<Vec<u64>> = (0..n_operations)
        .map(|col_idx| data.iter().map(|row| row[col_idx]).collect())
        .collect();

    let ops_sum_pt1 = do_ops(&ops, &transposed);

    // Part 2: Parse character by character
    let n_cols = data_lines.first().map(|l| l.len()).unwrap_or(0);
    let columns_string: Vec<String> = (0..n_cols)
        .map(|idx_op| {
            data_lines
                .iter()
                .filter_map(|line| line.chars().nth(idx_op))
                .collect()
        })
        .collect();

    // Splits numbers by operation
    let ops_numbers: Vec<Vec<u64>> = columns_string
        .split(|r| r.trim().is_empty())
        .map(|slice| {
            slice
                .iter()
                .filter_map(|x| x.trim().parse::<u64>().ok())
                .collect()
        })
        .collect();

    let ops_sum_pt2 = do_ops(&ops, &ops_numbers);

    (ops_sum_pt1.to_string(), ops_sum_pt2.to_string())
}

fn do_ops(ops: &Vec<char>, numbers: &Vec<Vec<u64>>) -> u64 {
    // Do operations on each row (originally columns)
    let ops_results: Vec<u64> = ops
        .iter()
        .zip(numbers.iter())
        .filter_map(|(op, nums)| {
            nums.iter()
                .copied()
                .reduce(|acc, n| if *op == '*' { acc * n } else { acc + n })
        })
        .collect();

    // Sum operations
    let ops_sum: u64 = ops_results.iter().sum();

    ops_sum
}

#[allow(dead_code)]
fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    println!("--");
    for row in matrix.iter() {
        for col in row.iter() {
            print!("{}, ", col);
        }
        println!("");
    }
    println!("--");
}
