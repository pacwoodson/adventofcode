use crate::utils::lines_to_vec;
extern crate nalgebra as na;

use na::{DMatrix, DVector};

type Lights = Vec<bool>;
type Button = Vec<usize>;
type Joltages = Vec<usize>;
type Machine = (Lights, Vec<Button>, Joltages);

fn buttons_match_light(machine: &Machine, buttons: &Vec<Button>) -> bool {
    let mut result_light = vec![false; machine.0.len()];
    for button in buttons.iter() {
        for i in button.iter() {
            result_light[*i] = !result_light[*i];
        }
    }

    *machine.0 == result_light
}

fn coefficients_joltage(joltages: &Joltages, buttons: &Vec<Button>) -> Option<Vec<u64>> {
    let n_counters = joltages.len();
    let n_buttons = buttons.len();

    let mut matrix_data: Vec<f32> = Vec::with_capacity(n_counters * n_buttons);

    for counter_idx in 0..n_counters {
        for button in buttons.iter() {
            if button.contains(&counter_idx) {
                matrix_data.push(1.0);
            } else {
                matrix_data.push(0.0);
            }
        }
    }

    let a_matrix = DMatrix::from_row_slice(n_counters, n_buttons, &matrix_data);
    let b_vector = DVector::from_vec(joltages.iter().map(|j| *j as f32).collect());

    let svd = a_matrix.clone().svd(true, true);

    let sol = svd.solve(&b_vector, 1e-4).ok();

    if let Some(res) = sol {
        let all_integers = res
            .iter()
            .all(|&x| x >= -1e-2 && (x - x.round()).abs() < 1e-2);

        if all_integers {
            let result = &a_matrix * &res;
            let is_valid = result
                .iter()
                .zip(b_vector.iter())
                .all(|(r, b)| (r - b).abs() < 1e-2);

            if is_valid {
                return Some(res.iter().map(|&x| x.round() as u64).collect());
            }
        }
    }
    None
}

fn combinations<T: Clone>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut c: Vec<Vec<T>> = vec![];

    let n = list.len();

    for i in 1..(2usize.pow(n as u32)) {
        let mut current: Vec<T> = vec![];
        for j in 0..n {
            if (i & (1usize << j)) != 0 {
                current.push(list[j].clone());
            }
        }
        c.push(current);
    }

    c.sort_by(|a, b| a.len().cmp(&b.len()));

    c
}

pub fn solve(input: &String, _: bool) -> (String, String) {
    let machines: Vec<Machine> = lines_to_vec(input)
        .iter()
        .map(|l| {
            let items: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();

            let lights_str = &items[0];
            let lights: Lights = lights_str[1..lights_str.len() - 1]
                .chars()
                .map(|c| if c == '#' { true } else { false })
                .collect();

            let buttons: Vec<Button> = items[1..items.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|b_n| b_n.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltages_str = &items[items.len() - 1];
            let joltages: Joltages = joltages_str[1..joltages_str.len() - 1]
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();

            (lights, buttons, joltages)
        })
        .collect();

    let part1: u64 = machines
        .iter()
        .map(|machine| {
            let combs = combinations(&machine.1);
            for comb in combs.iter() {
                if buttons_match_light(machine, &comb) {
                    return comb.len() as u64;
                }
            }
            panic!("Not found");
        })
        .sum();

    let part2: u64 = machines
        .iter()
        .filter_map(|machine| {
            combinations(&machine.1)
                .iter()
                .filter_map(|combo| {
                    coefficients_joltage(&machine.2, combo)
                        .map(|solution| solution.iter().sum::<u64>())
                })
                .min()
        })
        .sum();

    (part1.to_string(), part2.to_string())
}
