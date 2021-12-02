mod y2021;

use std::env;

fn main() {
    let day = env::args().nth(1).unwrap().parse::<i32>().unwrap();
    match day {
        1 => y2021::day1::solve(),
        2 => y2021::day2::solve(),
        _ => panic!("Don't know how to solve")
    };
}
