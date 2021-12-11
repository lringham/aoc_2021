use super::helpers;

enum Direction {
    Forward,
    Down,
    Up,
    Invalid,
}

impl Direction {
    fn from(name: &str) -> Self {
        match name {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::Invalid,
        }
    }
}

pub fn day2() {
    // parse input
    let lines = helpers::read_file("inputs/day2.txt");
    let mut directions: Vec<(Direction, i32)> = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        directions.push((
            Direction::from(values[0]),
            values[1].parse::<i32>().unwrap(),
        ));
    }

    // part 1
    // follow directions
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (direction, distance) in &directions {
        match direction {
            Direction::Forward => x += distance,
            Direction::Down => y += distance,
            Direction::Up => y -= distance,
            _ => {}
        }
    }
    println!("# part 1 - Ans: {}", x * y); // expected 1459206

    // part 2
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for (direction, distance) in &directions {
        match direction {
            Direction::Forward => {
                x += distance;
                y += aim * distance;
            }
            Direction::Down => aim += distance,
            Direction::Up => aim -= distance,
            _ => {}
        }
    }
    println!("# part 2 - Ans: {}", x * y); // expected 1320534480
}
