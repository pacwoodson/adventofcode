use crate::utils;

pub fn solve_fp(input: &String) -> (String, String) {
    let (sum_repeats_two, sum_repeats_more): (u64, u64) = input
        .as_str()
        .split(',')
        .map(utils::parse_range)
        .map(|range| {
            let mut range_sum_repeat_two: u64 = 0;
            let mut range_sum_repeat_more: u64 = 0;

            for i in range[0]..(range[1] + 1) {
                let id_string = format!("{}", i);

                if repeats_two(&id_string) {
                    range_sum_repeat_two += i as u64;
                }
                if repeats_more(&id_string) {
                    range_sum_repeat_more += i as u64;
                }
            }
            (range_sum_repeat_two, range_sum_repeat_more)
        })
        .fold((0, 0), |(s1, s2), (a1, a2)| (s1 + a1, s2 + a2));

    (sum_repeats_two.to_string(), sum_repeats_more.to_string())
}

fn repeats_two(s: &str) -> bool {
    let m = s.len() / 2;
    let halves = s.split_at(m);

    halves.0 == halves.1
}

fn repeats_more(s: &str) -> bool {
    // println!("{}", s);
    for chunk_size in 1..(s.len() / 2 + 1) {
        if s.len().rem_euclid(chunk_size) != 0 {
            continue;
        };
        // println!("chunk_size: {}", chunk_size);

        let mut prev = &s[0..chunk_size];
        let mut failed = false;
        for chunk_index in 1..(s.len() / chunk_size) {
            let chunk_pos = chunk_index * chunk_size;
            let curr = &s[chunk_pos..(chunk_pos + chunk_size)];
            // println!("prev: {}, curr: {}", prev, curr);

            if !prev.eq(curr) {
                failed = true;
                break;
            }
            prev = curr;
        }

        if failed {
            continue;
        }
        // println!("found");
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeats_more() {
        assert_eq!(repeats_more("11111"), true);
        assert_eq!(repeats_more("123123"), true);
        assert_eq!(repeats_more("123123123"), true);
        assert_eq!(repeats_more("121212"), true);
        assert_eq!(repeats_more("5252"), true);
        assert_eq!(repeats_more("525252"), true);
        assert_eq!(repeats_more("12341234"), true);

        assert_eq!(repeats_more("11112"), false);
        assert_eq!(repeats_more("123223"), false);
        assert_eq!(repeats_more("123123223"), false);
        assert_eq!(repeats_more("121312"), false);
        assert_eq!(repeats_more("5253"), false);
        assert_eq!(repeats_more("525251"), false);
        assert_eq!(repeats_more("12343234"), false);
    }
}

#[allow(dead_code)]
pub fn solve_ip(input: &String) -> (String, String) {
    let ranges: Vec<&str> = input.as_str().split(',').collect();

    let mut sum: u32 = 0;

    for range_str in ranges {
        let range_str_v: Vec<&str> = range_str.split('-').collect();
        let range: (u32, u32) = (
            range_str_v[0].parse().expect("parse range 0"),
            range_str_v[1].parse().expect("parse range 1"),
        );
        for i in range.0..(range.1 + 1) {
            let is = format!("{}", i);
            let m = is.len() / 2;
            let l = &is[..m];
            let r = &is[m..];
            let sym = l == r;

            if sym {
                sum += i;
            }
            // println!("{} {}", is, sym);
        }
    }

    (sum.to_string(), 0.to_string())
}

pub fn solve(input: &String) -> (String, String) {
    solve_fp(input)
}
