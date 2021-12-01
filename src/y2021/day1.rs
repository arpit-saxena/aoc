use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<i32> {
    let file = File::open("inputs/2021/1.txt").expect("Expected input file for day 1 as input1.txt");
    let mut vec = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            vec.push(l.parse::<i32>().unwrap());
        }
    }
    vec
}

fn solve_part1() {
    let input = get_input();
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            count += 1;
        }
    }
    println!("Answer for part 1:\n\t{}", count);
}

fn solve_part2() {
    let input = get_input();
    let mut count = 0;
    for i in 3..input.len() {
        if input[i] > input[i-3] {
            count += 1;
        }
    }
    println!("Answer for part 2:\n\t{}", count);
}