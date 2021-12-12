use super::helpers;

pub fn day11() {
    let lines = helpers::read_file("inputs/day11.txt");
    let mut numbers: Vec<Vec<u32>> = lines
        .iter()
        .map(|list| {
            list.chars()
                .map(|value| value.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let height = numbers.len();
    let width = numbers[0].len();
    let mut flash_count = 0;
    for step in 0..10000 {
        let mut will_flash: Vec<(usize, usize)> = Vec::new();

        // step 1 - add 1 to everyone and record flashers
        for (y, row) in &mut numbers.iter_mut().enumerate() {
            for (x, octopus) in row.iter_mut().enumerate() {
                *octopus += 1;

                if *octopus == 10 {
                    will_flash.push((x, y));
                }
            }
        }

        // step 2 - flash
        while will_flash.len() > 0 {
            let location = will_flash.pop().unwrap();
            let neighbours = get_neighbours(location.0, location.1, width, height);
            for neighbour_pos in neighbours {
                numbers[neighbour_pos.1][neighbour_pos.0] += 1;
                if numbers[neighbour_pos.1][neighbour_pos.0] == 10 {
                    will_flash.push(neighbour_pos);
                }
            }
        }

        // step 3 - set flashers to 0 and count flashes
        let mut cur_flash_count = 0;
        for row in &mut numbers {
            for octopus in row {
                if *octopus > 9 {
                    *octopus = 0;
                    cur_flash_count += 1;
                }
            }
        }
        flash_count += cur_flash_count;

        if step == 100 {
            println!("# part 1 - Ans: {}", flash_count);
        }

        if cur_flash_count == width * height {
            println!("# part 2 - Ans: {}", step + 1);
            break;
        }
    }
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let has_left = x != 0;
    let has_right = x < width - 1;
    let has_up = y < height - 1;
    let has_down = y != 0;

    if has_left {
        neighbours.push((x - 1, y));

        if has_up {
            neighbours.push((x - 1, y + 1));
        }
        if has_down {
            neighbours.push((x - 1, y - 1));
        }
    }
    if has_right {
        neighbours.push((x + 1, y));

        if has_up {
            neighbours.push((x + 1, y + 1));
        }
        if has_down {
            neighbours.push((x + 1, y - 1));
        }
    }
    if has_up {
        neighbours.push((x, y + 1));
    }
    if has_down {
        neighbours.push((x, y - 1));
    }

    neighbours
}
