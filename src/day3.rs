use super::helpers;

fn count_bit_occurances(list_of_numbers: &[usize], num_bits: usize) -> Vec<usize> {
    let mut counts = vec![0; num_bits];
    for value in list_of_numbers {
        for i in 0..num_bits {
            if (value >> i) & 0x1 > 0 {
                counts[num_bits - 1 - i] += 1;
            }
        }
    }
    counts
}

pub fn day3() {
    // parse input
    let lines = helpers::read_file("inputs/day3.txt");
    let num_bin_nums = lines.len();
    let num_bits = lines[0].len();

    let numbers: Vec<usize> = lines
        .iter()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();

    // part 1
    let mut counts = count_bit_occurances(&numbers, num_bits);
    let mut gamma = 0;
    let mut alpha = 0;
    for i in 0..num_bits {
        if counts[num_bits - 1 - i] > num_bin_nums / 2 {
            gamma += 0x1 << i;
        } else {
            alpha += 0x1 << i;
        }
    }
    println!("part 1 - Ans: {}", gamma * alpha); // expected 2003336

    // part 2
    let mut candidate_nums;
    let mut ratings = [0; 2];
    for rating_bit in [1, 0] {
        candidate_nums = numbers.clone();
        counts = count_bit_occurances(&candidate_nums, num_bits);

        for i in 0..counts.len() {
            // initialize candidate numbers
            if candidate_nums.len() == 1 {
                break;
            }

            let count = counts[i];
            let mut next_candidate_nums: Vec<usize> = Vec::new();

            for value in &candidate_nums {
                let bit_value = (value >> (num_bits - 1 - i)) & 0x1;
                let is_rating_bit = bit_value == rating_bit;
                let is_1_most_common = count * 2 >= candidate_nums.len();

                if is_1_most_common && is_rating_bit {
                    next_candidate_nums.push(*value);
                } else if !is_1_most_common && !is_rating_bit {
                    next_candidate_nums.push(*value);
                }
            }

            // update candidate_nums and recount bits
            if !next_candidate_nums.is_empty() {
                candidate_nums.clear();

                for value in next_candidate_nums {
                    candidate_nums.push(value);
                }
                counts = count_bit_occurances(&candidate_nums, num_bits);
            }
        }
        ratings[rating_bit] = candidate_nums[0];
    }
    println!(
        "part 2 - Ans (Life support rating): {}",
        ratings[0] * ratings[1]
    ); // expected 1877139
}
