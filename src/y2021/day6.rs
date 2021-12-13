use std::fs::File;
use std::io::Read;

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<i32> {
    let mut file = File::open("inputs/2021/6.txt").expect("Expected input file for day 6 as 6.txt");

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve_part1() {
    let mut fishes = get_input();

    for _ in 0..80 {
        let num_new_fish = fishes.iter().filter(|t| **t == 0).count();
        fishes
            .iter_mut()
            .for_each(|t| if *t == 0 { *t = 6 } else { *t -= 1 });
        fishes.extend(vec![8; num_new_fish]);
    }

    println!("Part 1:\n\t{}", fishes.len());
}

fn solve_part2() {
    let mut dp = vec![vec![0 as i64; 9]; 257];
    for days_left in 1..=256 {
        for timer in 0..=8 {
            let num_fish = if timer == 0 {
                1 + dp[days_left - 1][8] + dp[days_left - 1][6]
            } else {
                dp[days_left - 1][timer - 1]
            };

            dp[days_left][timer] = num_fish;
        }
    }

    let fishes = get_input();
    let total_fishes = fishes.iter().fold(0 as i64, |cum, timer| {
        cum + 1 + dp[256][*timer as usize]
    });

    println!("Part 2:\n\t{}", total_fishes);
}
