#![allow(dead_code)]
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
        7 => day7::day7(),
        8 => day8::day8(),
        9 => day9::day9(),
        10 => day10::day10(),
        11 => day11::day11(),
        12 => day12::day12(),
        _ => println!("Invalid command line args!\nUsage: cargo r day_num [1-25]"),
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
