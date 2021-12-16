use super::helpers;
use std::collections::HashMap;

pub fn day14() {
    // parse input
    let lines = helpers::read_file("inputs/day14.txt");
    let mut template: Vec<char> = Vec::new();
    let mut instructions = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            template = line.chars().collect();
        } else if i > 1 {
            let mut line_iter = line.split(" -> ");
            instructions.insert(
                line_iter.next().unwrap().to_string(),
                line_iter.next().unwrap().to_string(),
            );
        }
    }

    // part 1 ---
    let part_1 = simulate(template.clone(), &instructions, 10);
    let part_1_counts = count_letters(&part_1);
    let mut max_score = usize::MIN;
    let mut min_score = usize::MAX;
    for (_, v) in part_1_counts {
        max_score = if v > max_score { v } else { max_score };
        min_score = if v < min_score { v } else { min_score };
    }
    println!("# part 1 - Ans: {:?}", max_score - min_score); // expected 2740

    // part 2 ---
    // find string after 20 iterations
    let mut count_memo: HashMap<String, HashMap<char, usize>> = HashMap::new();
    template = simulate(template, &instructions, 20);

    // for each par of characters at iter=20, if they have been recorded and saved before
    // get count from memo, else simulate them for 20 iterations and
    // record the result into memo
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in &template {
        counts.insert(*ch, 0);
    }

    for i in 0..template.len() - 1 {
        let mut key = "".to_string();
        key.push(template[i]);
        key.push(template[i + 1]);

        if !count_memo.contains_key(&key) {
            let key_chars = key.chars().collect();
            let chars = simulate(key_chars, &instructions, 20);
            let letter_count_hashmap = count_letters(&chars);
            count_memo.insert(key.clone(), letter_count_hashmap);
        }

        let char_counts = &count_memo[&key];
        for (k, v) in char_counts {
            *counts.get_mut(&k).unwrap() += v;
        }

        if i > 0 {
            *counts.get_mut(&template[i]).unwrap() -= 1;
        }
    }

    // find max and min values
    let mut max_score = usize::MIN;
    let mut min_score = usize::MAX;
    for (_, v) in counts {
        max_score = if v > max_score { v } else { max_score };
        min_score = if v < min_score { v } else { min_score };
    }

    println!("# part 2 - Ans: {:?}", max_score - min_score); // expected 2,959,788,056,211
}

fn simulate(
    start_string: Vec<char>,
    instructions: &HashMap<String, String>,
    num_steps: usize,
) -> Vec<char> {
    let mut result = start_string;
    for _ in 0..num_steps {
        let mut new_template: String = "".to_string();
        let mut last_matched = false;
        for i in 0..result.len() - 1 {
            let mut key: String = "".to_string();
            key.push(result[i]);
            key.push(result[i + 1]);

            if instructions.contains_key(&key) {
                if !last_matched {
                    new_template.push(result[i]);
                }
                let ch = instructions[&key].chars().collect::<Vec<char>>();
                let ch = ch[0];
                new_template.push(ch);
                new_template.push(result[i + 1]);
                last_matched = true;
            } else {
                last_matched = false;
            }
        }
        result = new_template.chars().collect::<Vec<char>>();
    }

    result
}

fn count_letters(letters: &Vec<char>) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for letter in letters {
        let letter_str = letter;
        if counts.contains_key(&letter_str) {
            *counts.get_mut(&letter_str).unwrap() += 1;
        } else {
            counts.insert(*letter_str, 1);
        }
    }
    counts
}
