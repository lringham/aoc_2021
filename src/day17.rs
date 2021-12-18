use super::helpers;
// use std::cmp::Ordering;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::collections::HashMap;
// use std::collections::HashSet;

pub fn day17() {
    let lines = helpers::read_file("inputs/day17.txt");
    let target_coords: Vec<&str> = lines[0]
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ")
        .collect();

    let mut target = Target::new();
    let coord = parse_coords(target_coords[0]);
    target.low[0] = coord[0];
    target.high[0] = coord[1];

    let coord = parse_coords(target_coords[1]);
    target.low[1] = coord[0];
    target.high[1] = coord[1];

    let mut probe = Probe::new();
    let mut num_valid_init_conds = 0;
    let mut max_y = 0;
    for y_vel in -200..200 {
        for x_vel in 1..200 {
            probe.reset();
            probe.vel[0] = x_vel;
            probe.vel[1] = y_vel;
            let mut path : Vec<(i32, i32)> = Vec::new();
            loop {
                simulate(&mut probe);
                let pos = (probe.pos[0], probe.pos[1]);
                path.push(pos);
                let target_hit = is_in_target(pos, &target);
                if target_hit || probe.pos[1] < target.low[1] {
                    if target_hit {
                        // record the max y value for part 1
                        for (_, y) in path {
                            max_y = if y > max_y {y} else {max_y};
                        }
                        // count num valid initial conditions for part 2
                        num_valid_init_conds += 1;
                    }
                    break;
                }
            }
        }
    }

    println!("# part 1 - Ans: {}", max_y); // expected 5565
    println!("# part 2 - Ans: {}", num_valid_init_conds); // expected 2118
}

fn parse_coords(coords_str : &str) -> Vec<i32> {
    coords_str
        .split('=')
        .skip(1)
        .collect::<String>()
        .split("..")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

struct Target {
    pub low: Vec<i32>,
    pub high: Vec<i32>,
}

impl Target {
    pub fn new() -> Self {
        Target {
            low: vec![0; 2],
            high: vec![0; 2],
        }
    }
}

struct Probe {
    pub pos: Vec<i32>,
    pub vel: Vec<i32>,
}

impl Probe {
    pub fn new() -> Self {
        Probe {
            pos: vec![0; 2],
            vel: vec![0; 2],
        }
    }

    pub fn reset(&mut self) {
        self.pos[0] = 0;
        self.pos[1] = 0;
        self.vel[0] = 0;
        self.vel[1] = 0;
    }
}

fn drag(val: i32) -> i32 {
    if val < 0 {
        -1
    } else if val > 0 {
        1
    } else {
        0
    }
}

fn simulate(probe: &mut Probe) {
    probe.pos[0] += probe.vel[0];
    probe.pos[1] += probe.vel[1];

    probe.vel[0] -= drag(probe.vel[0]);
    probe.vel[1] -= 1; // gravity
}

fn is_in_target((x, y): (i32, i32), target: &Target) -> bool {
    x >= target.low[0] && x <= target.high[0] && y >= target.low[1] && y <= target.high[1]
}

fn draw(states: &Vec<(i32, i32)>, target: &Target) {
    let mut min = vec![i32::MAX; 2];
    let mut max = vec![i32::MIN; 2];
    for (x, y) in states {
        min[0] = if *x < min[0] { *x } else { min[0] };
        min[1] = if *y < min[1] { *y } else { min[1] };
        max[0] = if *x > max[0] { *x } else { max[0] };
        max[1] = if *y > max[1] { *y } else { max[1] };
    }
    min[0] = if target.low[0] < min[0] {
        target.low[0]
    } else {
        min[0]
    };
    min[1] = if target.low[1] < min[1] {
        target.low[1]
    } else {
        min[1]
    };
    max[0] = if target.high[0] > max[0] {
        target.high[0]
    } else {
        max[0]
    };
    max[1] = if target.high[1] > max[1] {
        target.high[1]
    } else {
        max[1]
    };

    for y in (min[1]..=max[1]).rev() {
        for x in min[0]..=max[0] {
            if states.contains(&(x, y)) {
                print!("#");
            } else {
                if is_in_target((x, y), &target) {
                    print!("T");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}
