use super::helpers;

pub fn day13() {
    let lines = helpers::read_file("inputs/day13.txt");

    let mut dots: Vec<(usize, usize)> = Vec::new();
    let mut instructions: Vec<(char, usize)> = Vec::new();
    let mut empty_line_seen = false;
    for line in &lines {
        if line.eq("") {
            empty_line_seen = true;
            continue;
        }
        if empty_line_seen {
            let line_chars: Vec<char> = line.chars().collect();
            let mut prev_c = '\0';
            let mut axis = '\0';
            let mut value: Vec<char> = Vec::new();
            for c in line_chars {
                if c == '=' {
                    axis = prev_c;
                } else if axis != '\0' {
                    value.push(c);
                }
                prev_c = c;
            }
            let s: String = value.into_iter().collect();
            instructions.push((axis, s.parse::<usize>().unwrap()));
        } else {
            let mut num_str = line.split(',');
            dots.push((
                num_str.next().unwrap().parse::<usize>().unwrap(),
                num_str.next().unwrap().parse::<usize>().unwrap(),
            ));
        }
    }

    // fold the paper
    for instruction in instructions {
        if instruction.0 == 'y' {
            for dot in &mut dots {
                if dot.1 > instruction.1 {
                    dot.1 -= 2 * (dot.1 - instruction.1);
                }
            }
        } else if instruction.0 == 'x' {
            for dot in &mut dots {
                if dot.0 > instruction.1 {
                    dot.0 -= 2 * (dot.0 - instruction.1);
                }
            }
        }
    }

    // let mut ans = 0;
    for y in 0..6 {
        for x in 0..39 {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
