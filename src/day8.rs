use super::helpers;
use std::collections::HashMap;

pub fn intersect_strs(str1: &String, str2: &String) -> String {
    let mut intersect: Vec<char> = Vec::new();

    for c1 in str1.chars() {
        if str2.contains(c1) {
            intersect.push(c1);
        }
    }

    intersect.iter().collect()
}

pub fn difference_strs(str1: &String, str2: &String) -> String {
    let mut difference: Vec<char> = Vec::new();

    for c in str1.chars() {
        if !str2.contains(c) {
            difference.push(c);
        }
    }

    for c in str2.chars() {
        if !str1.contains(c) {
            difference.push(c);
        }
    }

    difference.iter().collect()
}

pub fn day8() {
    let lines = helpers::read_file("inputs/day8.txt");
    let mut ans = 0;
    for line in &lines {
        let input: Vec<&str> = line.split('|').collect();
        let input: Vec<Vec<&str>> = input
            .iter()
            .map(|s| s.split_whitespace().collect::<Vec<&str>>())
            .collect();

        for entry in input[1].iter() {
            match entry.len() {
                2 => ans += 1,
                4 => ans += 1,
                3 => ans += 1,
                7 => ans += 1,
                _ => {}
            };
        }
    }

    println!("# part 1 - Ans: {}", ans); // expected 352

    let top: u8 = 0;
    let top_left: u8 = 1;
    let top_right: u8 = 2;
    let middle: u8 = 3;
    let bottom_left: u8 = 4;
    let bottom_right: u8 = 5;
    let bottom: u8 = 6;

    let mut numbers: Vec<usize> = Vec::new();
    for line in lines {
        let input: Vec<&str> = line.split('|').collect();
        let input: Vec<Vec<&str>> = input
            .iter()
            .map(|s| s.split_whitespace().collect::<Vec<&str>>())
            .collect();

        let one_seg_chars = input[0].iter().find(|s| s.len() == 2).unwrap().to_string();
        let seven_seg_chars = input[0].iter().find(|s| s.len() == 3).unwrap().to_string();
        let four_seg_chars = input[0].iter().find(|s| s.len() == 4).unwrap().to_string();
        let eight_seg_chars = input[0].iter().find(|s| s.len() == 7).unwrap().to_string();

        let mut two_seg_chars = String::new();
        let mut three_seg_chars = String::new();
        let mut nine_seg_chars = String::new();

        // top = one_seg_chars - seven_seg_chars
        let mut display_list = input[0].clone();
        display_list.retain(|s| s.len() != 2 && s.len() != 3 && s.len() != 4 && s.len() != 7);

        let mut char_map: HashMap<u8, char> = HashMap::new();

        // find top chars and top
        let top_chars: Vec<char> = seven_seg_chars
            .chars()
            .filter(|&ch| !one_seg_chars.contains(ch))
            .collect();
        char_map.insert(top, top_chars[0]);

        // find nine chars and bottom
        let temp_chars = [four_seg_chars.to_string(), char_map[&top].to_string()].concat();
        for i in 0..display_list.len() {
            let x = display_list[i];
            let difference = difference_strs(&x.to_string(), &temp_chars);
            if difference.len() == 1 {
                nine_seg_chars = [difference.to_string(), temp_chars.to_string()].concat();
                char_map.insert(bottom, difference.chars().next().unwrap());
                display_list.remove(i);
                break;
            }
        }

        // find three chars and the middle
        let temp_chars = [seven_seg_chars.to_string(), char_map[&bottom].to_string()].concat();
        for i in 0..display_list.len() {
            let x = display_list[i];
            let difference = difference_strs(&x.to_string(), &temp_chars);
            if difference.len() == 1 {
                three_seg_chars = [difference.to_string(), temp_chars.to_string()].concat();
                char_map.insert(middle, difference.chars().next().unwrap());
                display_list.remove(i);
                break;
            }
        }

        // find bottom left
        let temp = difference_strs(&eight_seg_chars, &nine_seg_chars)
            .chars()
            .next()
            .unwrap();
        char_map.insert(bottom_left, temp);

        // find top left
        char_map.insert(
            top_left,
            difference_strs(
                &[one_seg_chars.clone(), char_map[&middle].to_string()].concat(),
                &four_seg_chars,
            )
            .chars()
            .next()
            .unwrap(),
        );

        // find two chars and top right
        let mut temp_chars: Vec<char> = vec!['\0'; 4];
        temp_chars[0] = char_map[&middle];
        temp_chars[1] = char_map[&top];
        temp_chars[2] = char_map[&bottom];
        temp_chars[3] = char_map[&bottom_left];
        for i in 0..display_list.len() {
            let x = display_list[i];
            let difference = difference_strs(&x.to_string(), &temp_chars.iter().collect());
            if difference.len() == 1 {
                two_seg_chars = [difference.to_string(), temp_chars.iter().collect()].concat();
                char_map.insert(top_right, difference.chars().next().unwrap());
                display_list.remove(i);
                break;
            }
        }
        let temp = difference_strs(&one_seg_chars, &char_map[&top_right].to_string())
            .chars()
            .next()
            .unwrap();
        char_map.insert(bottom_right, temp);

        // figure out five
        let mut five_seg_chars = nine_seg_chars.clone();
        five_seg_chars.retain(|f| f != char_map[&top_right]);
        display_list.retain(|s| !difference_strs(&s.to_string(), &five_seg_chars).is_empty());

        // figure out zero
        let mut zero_seg_chars = eight_seg_chars.clone();
        zero_seg_chars.retain(|f| f != char_map[&middle]);
        display_list.retain(|s| !difference_strs(&s.to_string(), &zero_seg_chars).is_empty());

        // figure out 6
        let mut six_seg_chars = eight_seg_chars.clone();
        six_seg_chars.retain(|f| f != char_map[&top_right]);
        display_list.retain(|s| !difference_strs(&s.to_string(), &six_seg_chars).is_empty());

        ans = 0;
        let base: usize = 10;
        for (i, entry) in input[1].iter().enumerate() {
            if difference_strs(&entry.to_string(), &one_seg_chars).is_empty() {
                ans += base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &two_seg_chars).is_empty() {
                ans += 2 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &three_seg_chars).is_empty() {
                ans += 3 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &four_seg_chars).is_empty() {
                ans += 4 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &five_seg_chars).is_empty() {
                ans += 5 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &six_seg_chars).is_empty() {
                ans += 6 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &seven_seg_chars).is_empty() {
                ans += 7 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &eight_seg_chars).is_empty() {
                ans += 8 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &nine_seg_chars).is_empty() {
                ans += 9 * base.pow(3 - i as u32);
            } else if difference_strs(&entry.to_string(), &zero_seg_chars).is_empty() {
                // always 0
            }
        }
        numbers.push(ans);
    }

    println!("# part 2 - Ans: {}", numbers.iter().sum::<usize>()); // expected 936117
}
