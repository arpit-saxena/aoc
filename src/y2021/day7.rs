use std::fs::File;
use std::io::Read;

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<i32> {
    let mut file = File::open("inputs/2021/7.txt").expect("Expected input file for day 7 as 7.txt");

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn solve_part1() {
    let mut positions = get_input();
    positions.sort();
    let len = positions.len();

    let mut total = 0;
    for i in 0..len / 2 {
        total += positions[len - 1 - i] - positions[i];
    }
    println!("Part 1:\n\t{}", total);
}

fn solve_part2() {
    let positions = get_input();

    let len = positions.len() as i32;
    let pos = (2 * positions.iter().sum::<i32>() - len) / (2 * len);

    let f = |pos: i32| {
        positions
            .iter()
            .map(|init| ((init - pos).abs() * ((init - pos).abs() + 1)) / 2)
            .sum::<i32>()
    };
    let min_cost = f(pos).min(f(pos + 1));

    println!("Part 2:\n\t{}", min_cost);
}
