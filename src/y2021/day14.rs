use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> (Vec<char>, HashMap<String, char>) {
    let file = File::open("inputs/2021/14.txt").expect("Expected input file for day 14 as 14.txt");
    let mut reader = BufReader::new(file);

    let mut template = String::new();
    reader.read_line(&mut template).unwrap();

    {
        let mut temp = String::new();
        reader.read_line(&mut temp).unwrap();
        assert_eq!(temp, "\n", "Expected new line");
    }

    let mut rules = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let (match_pair, insert_char): (String, char);
        scan!(line.bytes() => "{} -> {}", match_pair, insert_char);
        rules.insert(match_pair, insert_char);
    }

    (template.trim().chars().collect(), rules)
}

fn solve_part1() {
    let (mut template, rules) = get_input();

    for _ in 0..10 {
        let mut new_template = Vec::new();
        new_template.push(template[0]);
        for pair in template.windows(2) {
            if let Some(&insert) = rules.get(&pair.iter().collect::<String>()) {
                new_template.push(insert);
            }
            new_template.push(pair[1]);
        }

        template = new_template;
    }

    let mut freq_map = HashMap::new();
    for c in template {
        let count = freq_map.entry(c).or_insert(0);
        *count += 1;
    }

    let freqs = freq_map.iter().map(|p| p.1).sorted().collect::<Vec<_>>();

    println!("Part 1:\n\t{}", freqs[freqs.len() - 1] - freqs[0]);
}

fn solve_part2() {
    let mut future_gen: HashMap<Vec<char>, HashMap<char, i64>> = HashMap::new();
    let (template, rules) = get_input();

    for _ in 1..=40 {
        let mut new_future_gen: HashMap<Vec<char>, HashMap<char, i64>> = HashMap::new();
        for (pair, &insert) in &rules {
            let pair_vec = pair.chars().collect::<Vec<_>>();
            let pairs = vec![vec![pair_vec[0], insert], vec![insert, pair_vec[1]]];

            let mut freq_map = HashMap::from([(insert, 1 as i64)]);
            for pair in pairs {
                for (&c, &freq) in future_gen.get(&pair).unwrap_or(&HashMap::new()) {
                    *freq_map.entry(c).or_insert(0) += freq;
                }
            }

            new_future_gen.insert(pair_vec, freq_map);
        }
        future_gen = new_future_gen;
    }

    let mut freq_map = HashMap::from([(template[0], 1)]);
    for pair in template.windows(2) {
        for (&c, &freq) in future_gen.get(pair).unwrap_or(&HashMap::new()) {
            *freq_map.entry(c).or_insert(0) += freq;
        }
        *freq_map.entry(pair[1]).or_insert(0) += 1;
    }

    let freqs = freq_map
        .iter()
        .map(|(_, &v)| v)
        .sorted()
        .collect::<Vec<_>>();

    println!("Part 2:\n\t{}", freqs[freqs.len() - 1] - freqs[0]);
}
