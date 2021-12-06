use super::helpers;

pub fn day1() {
    let lines = helpers::read_file("inputs/day1.txt");
    let depths: Vec<i32> = lines.iter().map(|s| s.parse::<i32>().unwrap()).collect();

    // part 1
    println!("# part 1 - Ans: {}", helpers::count_increases(&depths)); // expected 1676

    // part 2
    let mut windows: Vec<i32> = Vec::new();
    for i in 0..depths.len() - 2 {
        windows.push(depths[i] + depths[i + 1] + depths[i + 2]);
    }
    println!("# part 2 - Ans: {}", helpers::count_increases(&windows)); // expected 1706
}
