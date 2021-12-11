use super::helpers;
use std::collections::HashMap;

pub fn day10() {
    let lines = helpers::read_file("inputs/day10.txt");

    let mut point_map = HashMap::new();
    point_map.insert('(', 1);
    point_map.insert('[', 2);
    point_map.insert('{', 3);
    point_map.insert('<', 4);

    let mut illegal_chars: Vec<char> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupted = false;
        let chars: Vec<char> = line.chars().collect();
        for ch in chars.iter() {
            if corrupted {
                break;
            }

            match ch {
                ')' => {
                    if stack.last().unwrap() != &'(' {
                        corrupted = true;
                        illegal_chars.push(*ch);
                    }
                    stack.pop();
                }
                ']' => {
                    if stack.last().unwrap() != &'[' {
                        corrupted = true;
                        illegal_chars.push(*ch);
                    }
                    stack.pop();
                }
                '}' => {
                    if stack.last().unwrap() != &'{' {
                        corrupted = true;
                        illegal_chars.push(*ch);
                    }
                    stack.pop();
                }
                '>' => {
                    if stack.last().unwrap() != &'<' {
                        corrupted = true;
                        illegal_chars.push(*ch);
                    }
                    stack.pop();
                }
                _ => stack.push(*ch),
            }
        }

        if !corrupted {
            // line is incomplete (all lines are either corrupt or incomplete)
            let mut score = 0;
            for i in 0..stack.len() {
                let ch = stack[stack.len() - 1 - i];
                score = score * 5 + point_map[&ch];
            }
            scores.push(score);
        }
    }

    // part 1
    let mut point_map = HashMap::new();
    point_map.insert(')', 3);
    point_map.insert(']', 57);
    point_map.insert('}', 1197);
    point_map.insert('>', 25137);
    let mut ans = 0;
    for ch in illegal_chars.iter() {
        ans += point_map[ch]
    }
    println!("# part 1 - Ans {}", ans); // expected 339477

    // part 2
    scores.sort_unstable();
    println!("# part 2 - Ans {}", scores.get(scores.len() / 2).unwrap()); // expected 3049320156
}
