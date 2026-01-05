use core::panic;

use crate::utils::lines_to_vec;

type Lights = Vec<bool>;
type Button = Vec<usize>;
type Machine = (Lights, Vec<Button>);

fn buttons_match_light(light: &Lights, buttons: &Vec<Button>) -> bool {
    let mut result_light = vec![false; light.len()];
    for button in buttons.iter() {
        for i in button.iter() {
            result_light[*i] = !result_light[*i];
        }
    }
    *light == result_light
}

fn combinations<T: Clone>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut c: Vec<Vec<T>> = vec![];

    let n = list.len();

    for i in 1..(2usize.pow(n as u32) - 1) {
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

            // let joltages = &items[items.len() - 1];
            (lights, buttons)
        })
        .collect();

    let part1: u64 = machines
        .iter()
        .map(|machine| {
            let combs = combinations(&machine.1);
            for comb in combs.iter() {
                if buttons_match_light(&machine.0, &comb) {
                    return comb.len() as u64;
                }
            }
            panic!("Not found");
        })
        .sum();
    
    (part1.to_string(), "0".to_string())
}
