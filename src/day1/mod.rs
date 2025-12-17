use crate::utils;

pub fn solve(input: &String) -> (String, String) {
    let rows = utils::lines_to_vec(&input);

    let mut curr: i32 = 50;
    let mut zeros_at_arrival: u32 = 0;
    let mut zeros_passed: u32 = 0;

    for row in rows {
        let sign = if row.starts_with('L') { -1 } else { 1 };
        let step: i32 = row[1..].parse().expect("Not a number!");

        let prev = curr;
        curr += step * sign;

        let arrival = curr.rem_euclid(100);
        if arrival == 0 {
            zeros_at_arrival += 1;
            zeros_passed += 1;
        }

        let mut pos_times_100: u32 = (curr / 100).abs() as u32;
        if curr < 0 && prev != 0 {
            pos_times_100 += 1;
        }
        if pos_times_100 > 0 && arrival == 0 {
            pos_times_100 -= 1;
        }

        zeros_passed += pos_times_100;

        curr = arrival;

        // println!(
        //     "prev: {}, move: {}, passed_zeros: {}",
        //     prev,
        //     n * sign,
        //     passed_zero
        // );
    }

    // println!("{}", zeros);

    (zeros_at_arrival.to_string(), zeros_passed.to_string())
}
