use crate::utils;

pub fn solve(input: &String) -> (String, String) {
    let lines = utils::lines_to_vec(input);
    let separator_pos = lines
        .iter()
        .position(|l| l == "")
        .expect("Separator not found");

    let fresh_ranges: Vec<Vec<u64>> = lines[0..separator_pos]
        .iter()
        .map(|s| utils::parse_range(s.as_str()))
        .collect();

    let ingredient_ids: Vec<u64> = lines[separator_pos + 1..lines.len()]
        .iter()
        .map(|s| s.parse::<u64>().expect("Unable to parse Id"))
        .collect();

    let mut n_ids_fresh = 0;
    // for range in fresh_ranges.iter() {
    //     println!("range: {}-{}", range[0], range[1]);
    // }
    for id in ingredient_ids {
        let fresh = is_fresh(&fresh_ranges, id);
        // println!("id: {} fresh: {}", id, fresh);
        if fresh {
            n_ids_fresh += 1;
        }
    }

    (n_ids_fresh.to_string(), "0".to_string())
}

fn is_fresh(ranges: &Vec<Vec<u64>>, id: u64) -> bool {
    for range in ranges {
        if id > range[0] && id <= range[1] {
            return true;
        }
    }

    false
}
