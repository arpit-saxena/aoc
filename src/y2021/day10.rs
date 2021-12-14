use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<String> {
    let file = File::open("inputs/2021/10.txt").expect("Expected input file of day 10 as 10.txt");
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

fn find_first_illegal_close(line: &String) -> (Option<char>, Vec<char>) {
    let close_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '{' || c == '[' || c == '<' {
            stack.push(c);
        } else {
            match stack.pop() {
                None => return (Some(c), stack),
                Some(open) => {
                    if c != *close_map.get(&open).unwrap() {
                        return (Some(c), stack);
                    }
                }
            }
        }
    }

    (None, stack)
}

fn solve_part1() {
    let answer: i32 = get_input()
        .iter()
        .map(|line| find_first_illegal_close(line).0)
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Only expected closing chars, got {}", c),
        })
        .sum();

    println!("Part 1:\n\t{}", answer);
}

fn solve_part2() {
    let scores: Vec<_> = get_input()
        .iter()
        .map(|line| find_first_illegal_close(line))
        .filter(|(o, _)| o.is_none())
        .map(|(_, stack)| {
            stack
                .iter()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("Expected only opening chars, got {}", c),
                })
                .rev()
                .fold(0 as i64, |init, num| init * 5 + num)
        })
        .sorted()
        .collect();

    println!("Part 2:\n\t{}", scores[scores.len() / 2]);
}
