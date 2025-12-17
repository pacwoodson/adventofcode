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

    let mut n_fresh_inventory = 0;
    for id in ingredient_ids {
        let fresh = is_fresh(&fresh_ranges, id);
        // println!("id: {} fresh: {}", id, fresh);
        if fresh {
            n_fresh_inventory += 1;
        }
    }

    let fresh_ranges_dedup = dedup_ranges(&fresh_ranges);

    // println!("fresh ranges filtered:");
    // for fresh_range in fresh_ranges_filtered.iter() {
    //     println!("{}-{}", fresh_range[0], fresh_range[1]);
    // }

    let n_fresh_ids: u64 = fresh_ranges_dedup
        .iter()
        .map(|range| range[1] - range[0] + 1)
        .sum();

    (n_fresh_inventory.to_string(), n_fresh_ids.to_string())
}

fn is_fresh(ranges: &Vec<Vec<u64>>, id: u64) -> bool {
    for range in ranges {
        if id >= range[0] && id <= range[1] {
            return true;
        }
    }

    false
}

fn dedup_ranges(ranges: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut ranges_deduplicated: Vec<Vec<u64>> = vec![];
    let mut curr_range: Vec<u64> = sorted_ranges[0].clone();

    for new_fresh_range in sorted_ranges.iter() {
        if new_fresh_range[0] <= curr_range[1] + 1 {
            curr_range[1] = curr_range[1].max(new_fresh_range[1]);
        } else {
            ranges_deduplicated.push(curr_range);
            curr_range = new_fresh_range.clone();
        }
    }
    ranges_deduplicated.push(curr_range);

    ranges_deduplicated
}
