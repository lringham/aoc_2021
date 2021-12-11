use super::helpers;

pub fn day7() {
    let lines = helpers::read_file("inputs/day7.txt");
    let positions: Vec<i64> = lines[0]
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("# part 1 - Ans: {}", solve(&positions, calc_score_part1)); // expected 347509
    println!("# part 2 - Ans: {}", solve(&positions, calc_score_part2)); // expected 98257206
}

fn solve(positions: &Vec<i64>, calc_score: fn(i64) -> i64) -> i64 {
    let mut min_fuel_cost: i64 = i64::MAX;
    for new_position in 0..100000 {
        let cost = positions
            .iter()
            .fold(0, |sum, v| sum + calc_score((v - new_position).abs()));
        if cost < min_fuel_cost {
            min_fuel_cost = cost;
        }
    }
    min_fuel_cost
}

fn calc_score_part1(position: i64) -> i64 {
    position
}

fn calc_score_part2(position: i64) -> i64 {
    position * (position + 1) / 2
}
