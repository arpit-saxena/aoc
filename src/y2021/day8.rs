use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open("inputs/2021/8.txt").expect("Expected input of day 8 as 8.txt");

    let mut input = Vec::new();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let components: Vec<_> = line.split("|").collect();
        assert!(
            components.len() == 2,
            "There should only be one | in a line"
        );

        let digits = components[0]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let display = components[1]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();

        input.push((digits, display));
    }

    input
}

fn solve_part1() {
    let num_unique: usize = get_input()
        .iter()
        .map(|p| {
            p.1.iter()
                .filter(|s| [2, 3, 4, 7].contains(&(s.len() as i32)))
                .count()
        })
        .sum();

    println!("Part 1\n\t{}", num_unique);
}

fn solve_part2() {
    let standard_digits = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let mut sorted_standard_digits = standard_digits.clone();
    sorted_standard_digits.sort();

    let input = get_input();
    let mut sum_display = 0;
    for (mut digits, display) in input {
        digits
            .iter_mut()
            .for_each(|s| *s = s.chars().sorted().collect());
        digits.sort();

        let mapping_gen = |perm: Vec<u8>| {
            move |c: char| ('a' as u8 + perm[(c as u8 - 'a' as u8) as usize]) as char
        };

        let mut actual_perm = vec![];
        for perm in (0..7).permutations(7) {
            let mapping = mapping_gen(perm.clone());
            let curr_digits = sorted_standard_digits
                .iter()
                .map(|s| s.chars().map(&mapping).sorted().collect::<String>())
                .sorted()
                .collect::<Vec<_>>();

            if curr_digits == digits {
                actual_perm = perm;
                break;
            }
        }

        assert_ne!(actual_perm.len(), 0, "No permutation matched!");

        let mut rev_perm = vec![100; 8];
        for (inp, out) in actual_perm.into_iter().enumerate() {
            rev_perm[out as usize] = inp as u8;
        }
        let rev_mapping = mapping_gen(rev_perm);

        let display_num = display
            .iter()
            .map(|word| {
                let to_find = word.chars().map(&rev_mapping).sorted().collect::<String>();
                standard_digits
                    .iter()
                    .find_position(|&&w| w == to_find)
                    .unwrap()
                    .0
            })
            .fold(0, |val, digit| val * 10 + digit);

        sum_display += display_num;
    }

    println!("Part 2:\n\t{}", sum_display);
}
