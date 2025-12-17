use crate::utils;

fn solve_fp(input: &String) -> (String, String) {
    let banks: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|bank_str| bank_str.trim().bytes().map(|b| b - b'0').collect())
        .collect();

    (
        sum_batteries(&banks, 2).to_string(),
        sum_batteries(&banks, 12).to_string(),
    )
}

fn sum_batteries(banks: &Vec<Vec<u8>>, battery_size: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_best_battery(&bank, battery_size))
        .sum()
}

fn find_best_battery(bank: &Vec<u8>, battery_size: usize) -> u64 {
    let mut joltage: u64 = 0;
    let mut current_index: usize = 0;

    for i in 0..battery_size {
        let max_index = bank.len() - (battery_size - i - 1);
        let (mut max_position, max_value) = bank[current_index..max_index]
            .iter()
            .rev() // Takes the first one with max
            .enumerate()
            .max_by_key(|e_digit| e_digit.1)
            .expect("this bank is empty");
        max_position = max_index - max_position - 1;

        // println!("{} {} @{}", row, max_value, max_position);

        joltage += (*max_value as u64) * 10u64.pow((battery_size - i - 1) as u32);
        current_index = max_position + 1;
    }

    joltage
}

#[allow(dead_code)]
fn solve_ip(input: &String) -> (String, String) {
    let mut sum: u32 = 0;

    let banks_str = utils::lines_to_vec(&input);
    let banks: Vec<Vec<u8>> = banks_str
        .iter()
        .map(|bank_str| {
            bank_str
                .as_bytes()
                .iter()
                .map(|b| {
                    str::from_utf8(&[*b])
                        .expect("rrrr1")
                        .parse::<u8>()
                        .expect("rrrr2")
                })
                .collect()
        })
        .collect();

    for bank in banks {
        let mut b1: u8 = 0;
        let mut ib1: usize = 0;
        let mut b2: u8 = 0;

        for i in 0..(bank.len() - 1) {
            let b = bank[i];
            if b > b1 {
                b1 = b;
                ib1 = i;
            }
        }
        for i in (ib1 + 1)..bank.len() {
            let b = bank[i];
            if b > b2 {
                b2 = b;
            }
        }

        let joltage: u32 = (b1 as u32) * 10 + (b2 as u32);
        // println!("{} {} {}", joltage, b1, b2);

        sum += joltage;
    }

    (sum.to_string(), 0.to_string())
}

pub fn solve(input: &String) -> (String, String) {
    solve_fp(input)
}
