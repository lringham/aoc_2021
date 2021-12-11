use super::helpers;

pub fn day5() {
    let lines = helpers::read_file("inputs/day5.txt");
    let mut numbers: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        let temp: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
        let temp_numbers: Vec<Vec<&str>> = temp
            .iter()
            .map(|s| s.split(',').collect::<Vec<&str>>())
            .collect();

        let temp_numbers: Vec<(usize, usize)> = temp_numbers
            .iter()
            .map(|pair| {
                (
                    pair[0].parse::<usize>().unwrap(),
                    pair[1].parse::<usize>().unwrap(),
                )
            })
            .collect();
        for x in temp_numbers {
            numbers.push(x);
        }
    }

    let max_number = numbers
        .iter()
        .reduce(|x, y| if y > x { y } else { x })
        .unwrap()
        .0;

    for part in [1, 2] {
        let mut vent_map = vec![vec![0; max_number + 1]; max_number + 1];

        for i in (0..numbers.len() - 1).step_by(2) {
            let start = numbers[i];
            let end = numbers[i + 1];

            let start_x = start.0;
            let end_x = end.0;

            let start_y = start.1;
            let end_y = end.1;

            let dx = end_x as i32 - start_x as i32;
            let dy = end_y as i32 - start_y as i32;

            if dx == 0 {
                if dy < 0 {
                    for y in end_y..start_y + 1 {
                        vent_map[y][start_x] += 1;
                    }
                } else {
                    for y in start_y..end_y + 1 {
                        vent_map[y][start_x] += 1;
                    }
                }
            } else if dy == 0 {
                if dx < 0 {
                    for x in end_x..start_x + 1 {
                        vent_map[start_y][x] += 1;
                    }
                } else {
                    for x in start_x..end_x + 1 {
                        vent_map[start_y][x] += 1;
                    }
                }
            } else if dx.abs() == dy.abs() && part == 2 {
                let x_vals: Vec<usize> = if dx > 0 {
                    (start_x..end_x + 1).collect()
                } else {
                    (end_x..start_x + 1).rev().collect()
                };

                let y_vals: Vec<usize> = if dy > 0 {
                    (start_y..end_y + 1).collect()
                } else {
                    (end_y..start_y + 1).rev().collect()
                };

                let coordinates: Vec<(usize, usize)> =
                    x_vals.into_iter().zip(y_vals.into_iter()).collect();
                for (x, y) in coordinates {
                    vent_map[y][x] += 1;
                }
            }
        }

        // count
        let danger_thresh = 2;
        let mut num_dangerous = 0;
        for x in vent_map {
            for v in x {
                if v >= danger_thresh {
                    num_dangerous += 1;
                }
            }
        }

        // print ans
        if part == 1 {
            println!("part 1 - Ans: {}", num_dangerous); // expected 5306
        } else if part == 2 {
            println!("part 2 - Ans: {}", num_dangerous); // expected 17787
        }
    }
}
