#![allow(dead_code)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod helpers;

use std::env;

fn main() {
    let day_num = what_day();
    println!("### Day {} ###", day_num);
    match day_num {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        _ => println!("Invalid command line args!\nUsage: cargo r day_num [1-31]"),
    }
}

fn what_day() -> i32 {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        -1
    } else {
        args[1].parse::<i32>().unwrap_or(-1)
    }
}
