use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn get_input() -> Vec<Command> {
    let file = File::open("inputs/2021/2.txt").expect("Expected input file for day 2 as input2.txt");
    let mut vec = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let command_vec = line.split(" ").collect::<Vec<&str>>();
        let command_arg = command_vec[1].parse::<i32>().unwrap();
        let comm = match command_vec[0] {
            "forward" => Command::Forward(command_arg),
            "down" => Command::Down(command_arg),
            "up" => Command::Up(command_arg),
            _ => panic!("Unknown command {}", command_vec[0]),
        };

        vec.push(comm);
    }
    vec
}

fn solve_part1() {
    let commands = get_input();
    let mut x = 0;
    let mut y = 0;
    for comm in commands {
        match comm {
            Command::Forward(x1) => x += x1,
            Command::Down(y1) => y += y1,
            Command::Up(y1) => y -= y1,
        };
    }
    println!("Part 1:\n\t{}", x * y);
}

fn solve_part2() {
    let commands = get_input();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for comm in commands {
        match comm {
            Command::Forward(x1) => {
                x += x1;
                y += aim * x1;
            },
            Command::Down(y1) => aim += y1,
            Command::Up(y1) => aim -= y1,
        };
    }
    println!("Part2:\n\t{}", x * y);
}