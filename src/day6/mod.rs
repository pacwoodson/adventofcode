use std::fmt::Display;

use crate::utils;

pub fn solve(input: &String) -> (String, String) {
    let lines = utils::lines_to_vec(input);

    // Process input matrix
    let mut data: Vec<Vec<u64>> = vec![];
    for (y, line) in lines.iter().enumerate() {
        if y == lines.len() - 1 {
            continue;
        }
        data.push(vec![]);
        for row_chunk in line.split(' ') {
            let parsed = row_chunk.parse::<u64>();
            if let Ok(n) = parsed {
                data[y].push(n);
            }
        }
    }
    // print_matrix(&data);

    // Get operators for each operation array
    let mut ops: Vec<char> = vec![];

    for row_chunk in lines[lines.len() - 1].split(' ') {
        if row_chunk == "+" || row_chunk == "*" {
            ops.push(row_chunk.as_bytes()[0] as char);
        }
    }

    let operations_length = data.len();
    let n_operations = data[0].len();

    // transpose input matrix to get each operation on rows
    let mut transposed_data = vec![vec![0; operations_length]; n_operations];
    for (y, row) in data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            transposed_data[x][y] = *col;
        }
    }
    // print_matrix(&transposed_data);

    // Do operations on each row (originally columns)
    let ops_results: Vec<u64> = ops
        .iter()
        .enumerate()
        .map(|(op_index, op)| {
            transposed_data[op_index]
                .iter()
                .copied()
                .reduce(|acc, n| if *op == '*' { acc * n } else { acc + n })
                .unwrap_or(0)
        })
        .collect();

    // Sum operations
    let ops_sum: u64 = ops_results.iter().sum();

    (ops_sum.to_string(), "0".to_string())
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
