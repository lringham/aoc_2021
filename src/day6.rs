use super::helpers;

pub fn day6() {
    // parse input
    let lines = helpers::read_file("inputs/day6.txt");
    let fish_orig: Vec<usize> = lines[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // for each fish aged 0..8 find how many fish it would
    // produce in 128 years (half way to 256)
    let mut count_after_128: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for j in 0..9 {
        let mut fish = vec![j];

        for _ in 0..128 {
            let mut new_fish_count = 0;
            let mut temp_fish: Vec<usize> = Vec::new();

            for age in fish {
                if age == 0 {
                    new_fish_count += 1;
                    temp_fish.push(6);
                } else {
                    temp_fish.push(age - 1);
                }
            }

            while new_fish_count > 0 {
                temp_fish.push(8);
                new_fish_count -= 1;
            }

            fish = temp_fish.clone();
        }
        count_after_128[j] = fish.len();
    }

    // find all fish and their ages at 128 years from the puzzle input
    let mut fish = fish_orig;
    for i in 0..128 {
        let mut new_fish_count = 0;
        let mut temp_fish: Vec<usize> = Vec::new();

        for age in fish {
            if age == 0 {
                new_fish_count += 1;
                temp_fish.push(6);
            } else {
                temp_fish.push(age - 1);
            }
        }

        while new_fish_count > 0 {
            temp_fish.push(8);
            new_fish_count -= 1;
        }

        if i == 79 {
            println!("# part 1 - Ans: {}", temp_fish.len()); // expected 343441
        }

        fish = temp_fish.clone();
    }

    // Find the number of fish at year 256 (128 years from the current population)
    // by using the precalculated counts found before
    let mut ans = 0;
    for i in fish {
        ans += count_after_128[i];
    }

    println!("# part 2 - Ans: {}", ans); // expected 1569108373832
}
