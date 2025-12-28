mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod utils;

const SOLVE_FNS: [fn(&String, bool) -> (String, String); 8] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
];

fn main() {
    let app_opts = utils::get_opts();
    let solve_fn = SOLVE_FNS[app_opts.day - 1];
    let day_input = utils::get_day_input(app_opts.day);

    let result = solve_fn(&day_input.input, false);
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
            let res_test = solve_fn(&day_input.test, true);
            assert_eq!(res_test.0, day_input.test_result[0], "Part1 error");
            assert_eq!(res_test.1, day_input.test_result[1], "Part2 error");
            print!("[TEST]");
            if day_input.test_result.len() > 2 {
                let res_input = solve_fn(&day_input.input, false);
                assert_eq!(res_input.0, day_input.test_result[2], "Real part1 error");
                if day_input.test_result.len() == 4 {
                    assert_eq!(res_input.1, day_input.test_result[3], "Real part2 error");
                }
                print!("[REAL]");
            }
            println!("Test for day {} passed.", day + 1);
        }
        println!("All tests passed ✅");
    }
}
