use super::helpers;

pub fn day4() {
    // parse input
    let lines = helpers::read_file("inputs/day4.txt");
    let numbers: Vec<usize> = lines[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    // create boards
    let mut start_idx = 0;
    for (i, line) in lines.iter().enumerate() {
        // skip list of numbers
        if i == 0 {
            continue;
        }

        // new boards are preceded by an empty line
        if line.trim() == "" {
            boards.push(Board::new());
            start_idx = i;
            continue;
        }

        // convert string to usize list and update row
        let y = i - start_idx - 1;
        let row_values: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        for (x, val) in row_values.iter().enumerate() {
            boards.last_mut().unwrap().put_num(x, y, *val);
        }
    }

    // mark boards and find winners
    let mut results: Vec<usize> = Vec::new();
    let mut boards_to_skip: Vec<usize> = Vec::new();
    for val in numbers {
        //println!("Calling {}", val);
        for (i, board) in boards.iter_mut().enumerate() {
            if !boards_to_skip.contains(&i) {
                board.put_mark(val);
                if board.is_winning() {
                    // println!(" -- we have a winner! ");
                    // println!("Final Score: {}", board.score());
                    results.push(board.score());
                    boards_to_skip.push(i);
                }
            }
        }
    }

    println!("part 1 - Ans: {}", results[0]); // expected 16716
    println!("part 2 - Ans: {}", results.last().unwrap()); // expected 4880
}

struct Board {
    pub rows: [[(usize, bool); 5]; 5],
    winning: bool,
    winning_num: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: [[(0, false); 5]; 5],
            winning: false,
            winning_num: 0,
        }
    }

    pub fn print(&self) {
        for (y, row) in self.rows.iter().enumerate() {
            print!("{} ", y);
            for (num, marked) in row {
                print!("| num: {}, marked: {} |", num, marked);
            }
            println!();
        }
    }

    pub fn score(&self) -> usize {
        let mut total_score = 0;
        for row in &self.rows {
            for (v, m) in row {
                if !m {
                    total_score += v;
                }
            }
        }
        total_score * self.winning_num
    }

    pub fn is_winning(&self) -> bool {
        self.winning
    }

    pub fn put_num(&mut self, x: usize, y: usize, num: usize) {
        self.rows[y][x].0 = num;
    }

    pub fn put_mark(&mut self, val: usize) {
        let mut updated_coords: Vec<(usize, usize)> = Vec::new();
        for (y, row) in self.rows.iter_mut().enumerate() {
            for (x, entry) in row.iter_mut().enumerate() {
                if entry.0 == val {
                    entry.1 = true;
                    updated_coords.push((x, y));
                }
            }
        }

        for (x, y) in updated_coords {
            self.update(x, y, val);
        }
    }

    fn update(&mut self, x: usize, y: usize, num: usize) {
        let mut found = true;
        // check x
        for temp_x in 0..5 {
            if !self.rows[y][temp_x].1 {
                found = false;
            }
        }

        if found {
            self.winning = true;
            self.winning_num = num;
            return;
        }

        // check y
        found = true;
        for temp_y in 0..5 {
            if !self.rows[temp_y][x].1 {
                found = false;
            }
        }

        if found {
            self.winning = true;
            self.winning_num = num;
            return;
        }

        // check diags
        if x == 2 && y == 2 {
            found = true;
            for temp_y in 0..5 {
                for temp_x in 0..5 {
                    if !self.rows[temp_y][temp_x].1 {
                        found = false;
                    }
                }
            }

            if found {
                self.winning = true;
                self.winning_num = num;
                return;
            }

            found = true;
            for temp_y in (0..5).rev() {
                for temp_x in 0..5 {
                    if !self.rows[temp_y][temp_x].1 {
                        found = false;
                    }
                }
            }

            if found {
                self.winning = true;
                self.winning_num = num;
            }
        }
    }
}
