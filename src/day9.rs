use super::helpers;

pub fn day9() {
    // parse input
    let lines = helpers::read_file("inputs/day9.txt");
    let mut height_map: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let numbers = line
            .chars()
            .map(|s| s.to_string().parse::<u8>().unwrap())
            .collect();
        height_map.push(numbers);
    }

    // part 1
    let mut ans = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if is_min(x, y, &height_map) {
                ans += height_map[y][x] as usize + 1;
            }
        }
    }
    println!("# part 1 - Ans: {:?}", ans); // expected 526

    // part 2
    let mut basin_sizes: Vec<usize> = Vec::new();
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if is_min(x, y, &height_map) {
                let basin_size = get_basin_size(x, y, &height_map);
                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort_unstable();
    let basin_sizes: Vec<&usize> = basin_sizes.iter().rev().collect();
    println!(
        "# part 2 - Ans: {:?}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    ); // expected 1123524
}

fn is_min(x: usize, y: usize, height_map: &[Vec<u8>]) -> bool {
    let val = height_map[y][x];

    let has_left = x != 0;
    let has_right = x < height_map[0].len() - 1;
    let has_up = y < height_map.len() - 1;
    let has_down = y != 0;

    let mut is_lowest = true;
    if has_left {
        is_lowest = is_lowest && val < height_map[y][x - 1];
    }
    if has_right {
        is_lowest = is_lowest && val < height_map[y][x + 1];
    }
    if has_up {
        is_lowest = is_lowest && val < height_map[y + 1][x];
    }
    if has_down {
        is_lowest = is_lowest && val < height_map[y - 1][x];
    }
    is_lowest
}

fn flood_fill(
    x: usize,
    y: usize,
    height_map: &[Vec<u8>],
    seen_positions: &mut Vec<(usize, usize)>,
) -> usize {
    let width = height_map[0].len();
    let height = height_map.len();
    let seen_before = seen_positions.contains(&(x, y));
    let is_nine = height_map[y][x] == 9;
    let mut size = 0;

    if !is_nine && !seen_before {
        size = 1;
        seen_positions.push((x, y));

        if x > 0 {
            size += flood_fill(x - 1, y, height_map, seen_positions);
        }
        if x < width - 1 {
            size += flood_fill(x + 1, y, height_map, seen_positions);
        }
        if y > 0 {
            size += flood_fill(x, y - 1, height_map, seen_positions);
        }
        if y < height - 1 {
            size += flood_fill(x, y + 1, height_map, seen_positions);
        }
    }

    size
}

fn get_basin_size(x: usize, y: usize, height_map: &[Vec<u8>]) -> usize {
    let mut seen_positions: Vec<(usize, usize)> = Vec::new();
    flood_fill(x, y, height_map, &mut seen_positions)
}
