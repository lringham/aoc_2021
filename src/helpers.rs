use std::fs;

pub fn count_increases(values: &[i32]) -> i32 {
    let mut num_increases = 0;
    let mut prev_value = values[0];
    for value in values {
        if prev_value < *value {
            num_increases += 1;
        }
        prev_value = *value;
    }
    num_increases
}

pub fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    contents.lines().map(|s| s.to_string()).collect()
}
