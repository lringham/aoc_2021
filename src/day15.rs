use super::helpers;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn day15() {
    let lines = helpers::read_file("inputs/day15.txt");
    let numbers: Vec<Vec<usize>> = lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let width = numbers[0].len();
    let height = numbers.len();
    
    let mut grid : Vec<Vec<usize>> = Vec::new();
    for y in 0..height*5 {
        grid.push(Vec::new());
        for x in 0..width*5 {
            let x_section = x / width;
            let y_section = y / height;
            let mut num = numbers[y%height][x%width] + x_section + y_section;
            num = num / 10 + num % 10;
            grid[y].push(num); 
        }
    }

    // find path
    let width = numbers[0].len();
    let height = numbers.len();
    let path = find_lowest_path(&grid, (0, 0), (width*5 - 1, height*5 - 1));
    
    // calc score
    let mut score = 0;
    for i in 1..path.len() {
        score += path[i];
    }

    println!("{:?}", score);
}

fn find_lowest_path(
    grid: &Vec<Vec<usize>>,
    (start_x, start_y): (usize, usize),
    (end_x, end_y): (usize, usize),
) -> Vec<usize> {
    let width = end_x - start_x + 1;
    let height = end_y - start_y + 1;
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    let mut frontier = BinaryHeap::new();

    // path find
    frontier.push(Reverse(Cell::new(start_x, start_y, grid[start_y][start_x])));
    cost_so_far.insert((start_x, start_y), 0);
    while frontier.len() > 0 {
        let cell = frontier.pop().unwrap().0;
        let (x, y) = cell.location;
        let neighbours = helpers::get_neighbours(x, y, width, height);

        if cell.location == (end_x, end_y) {
            break;
        }

        for (n_x, n_y) in neighbours {
            let new_cost = cost_so_far[&cell.location] + grid[n_y][n_x];
            if !cost_so_far.contains_key(&(n_x, n_y)) || new_cost < cost_so_far[&(n_x, n_y)] {
                cost_so_far.insert((n_x, n_y), new_cost);
                frontier.push(Reverse(Cell::new(n_x, n_y, new_cost)));
                came_from.insert((n_x, n_y), (x, y));
            }
        }
    }

    // construct the path
    let mut path: Vec<usize> = Vec::new();
    let mut location: (usize, usize) = (end_x, end_y);
    while location != (start_x, start_y) {
        path.push(grid[location.1][location.0]);
        location = came_from[&(location.0, location.1)];
    }
    path.push(grid[start_x][start_y]);
    path.into_iter().rev().collect()
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
    println!();
}

#[derive(Eq)]
struct Cell {
    pub value: usize,
    pub location: (usize, usize),
}

impl Cell {
    fn new(x: usize, y: usize, value: usize) -> Self {
        Cell {
            value,
            location: (x, y),
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
