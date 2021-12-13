use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<Vec<char>> {
    let file = File::open("inputs/2021/3.txt").expect("Expected input file for day 3 as 3.txt");
    return BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.chars().collect())
        .collect();
}

fn solve_part1() {
    let mut gamma = 0;
    let mut epsilon = 0;
    let nums = get_input();
    for i in 0..nums[0].len() {
        let mut freq1 = 0;
        for j in 0..nums.len() {
            if nums[j][i] == '1' {
                freq1 += 1;
            }
        }

        gamma *= 2;
        epsilon *= 2;
        if freq1 > nums.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("Part 1:\n\t{}", gamma * epsilon);
}

fn solve_part2() {
    let mut nums = get_input();

    // Max
    let mut i = 0;
    while nums.len() > 1 {
        let mut freq1 = 0;
        for j in 0..nums.len() {
            if nums[j][i] == '1' {
                freq1 += 1;
            }
        }

        // println!("{} / {}", freq1, nums.len());

        let remove0 = freq1 >= (nums.len() - freq1);
        nums.drain_filter(|num| {
            if remove0 {
                num[i] == '0'
            } else {
                num[i] == '1'
            }
        });
        // println!("{:?}", nums.iter().map(|arr| arr.iter().collect::<String>()).collect::<Vec<_>>());
        i += 1;
    }

    let mut oxygen = 0;
    for char in &nums[0] {
        oxygen *= 2;
        if *char == '1' {
            oxygen += 1;
        }
    }

    // Min
    nums = get_input();
    i = 0;
    while nums.len() > 1 {
        let mut freq1 = 0;
        for j in 0..nums.len() {
            if nums[j][i] == '1' {
                freq1 += 1;
            }
        }

        let remove0 = freq1 < (nums.len() - freq1);
        nums.drain_filter(|num| remove0 ^ (num[i] == '1'));
        i += 1;
    }

    let mut co2 = 0;
    for char in &nums[0] {
        co2 *= 2;
        if *char == '1' {
            co2 += 1;
        }
    }

    println!("Part 2:\n\t{}", oxygen * co2);
}
