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

pub fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let has_left = x != 0;
    let has_right = x < width - 1;
    let has_up = y != 0;
    let has_down = y < height - 1;

    if has_left {
        neighbours.push((x - 1, y));
    }
    if has_right {
        neighbours.push((x + 1, y));
    }
    if has_up {
        neighbours.push((x, y - 1));
    }
    if has_down {
        neighbours.push((x, y + 1));
    }

    neighbours
}
