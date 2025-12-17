mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

const SOLVE_FNS: [fn(&String) -> (String, String); 5] =
    [day1::solve, day2::solve, day3::solve, day4::solve, day5::solve];

fn main() {
    let app_opts = utils::get_opts();
    let solve_fn = SOLVE_FNS[app_opts.day - 1];
    let day_input = utils::get_day_input(app_opts.day);

    let result = solve_fn(&day_input.input);
    println!("Result: part1: {} part2: {}", result.0, result.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_days() {
        for day in 0..SOLVE_FNS.len() {
            let solve_fn = SOLVE_FNS[day];
            let day_input = utils::get_day_input(day + 1);
            let res = solve_fn(&day_input.test);
            assert_eq!(res.0, day_input.test_result[0], "Part1 error");
            assert_eq!(res.1, day_input.test_result[1], "Part2 error");
            println!("Test for day {} passed.", day + 1);
        }
        println!("All tests passed ✅");
    }
}
