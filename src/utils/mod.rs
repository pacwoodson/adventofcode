use std::env;
use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read input file")
}

pub fn lines_to_vec(content: &String) -> Vec<String> {
    content.lines().map(|line| line.to_string()).collect()
}

pub struct DayInput {
    pub input: String,
    #[cfg(test)]
    pub test: String,
    #[cfg(test)]
    pub test_result: Vec<String>,
}

pub fn get_day_input(day: usize) -> DayInput {
    let base_path = format!("src/day{}/", day);
    let input_path = format!("{}/input", base_path);
    #[cfg(test)]
    let input_test_path = format!("{}/input_test", base_path);
    #[cfg(test)]
    let input_test_result_path = format!("{}/input_test_result", base_path);

    DayInput {
        input: read_input(&input_path),
        #[cfg(test)]
        test: read_input(&input_test_path),
        #[cfg(test)]
        test_result: lines_to_vec(&read_input(&input_test_result_path)),
    }
}
pub struct AppOpts {
    pub day: usize,
}

pub fn get_opts() -> AppOpts {
    let args: Vec<String> = env::args().collect();

    let mut app_opts = AppOpts { day: 1 };

    for arg in &args[1..] {
        let parsed = arg.parse::<usize>();
        if parsed.is_ok() {
            app_opts.day = parsed.unwrap();
        } else {
            panic!("Unknown argument");
        }
    }

    app_opts
}
