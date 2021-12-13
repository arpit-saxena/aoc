use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

fn get_input() -> Vec<Line> {
    let file = File::open("inputs/2021/5.txt").expect("Expected input file for day 5 as 5.txt");
    let reader = BufReader::new(file);
    let mut lines = Vec::<Line>::new();

    for text_line in reader.lines() {
        let text_line = text_line.unwrap();
        let (x1, y1, x2, y2): (i32, i32, i32, i32);
        scan!(text_line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
        let line = Line {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        };
        lines.push(line);
    }

    lines
}

fn get_grid(lines: &Vec<Line>) -> Vec<Vec<i32>> {
    let (max_x, max_y) = lines.iter().fold((0, 0), |(mut max_x, mut max_y), line| {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            return (max_x, max_y);
        }

        max_x = max_x.max(line.start.x).max(line.end.x);
        max_y = max_y.max(line.start.y).max(line.end.y);

        (max_x, max_y)
    });

    let size_x: usize = (max_x + 1).try_into().unwrap();
    let size_y: usize = (max_y + 1).try_into().unwrap();

    vec![vec![0; size_y]; size_x]
}

fn solve_part1() {
    let lines = get_input();
    let mut grid = get_grid(&lines);

    for line in lines {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            continue;
        }

        for x in line.start.x.min(line.end.x)..=line.end.x.max(line.start.x) {
            for y in line.start.y.min(line.end.y)..=line.end.y.max(line.start.y) {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    let mut count = 0;
    for row in grid {
        for num in row {
            if num >= 2 {
                count += 1;
            }
        }
    }

    println!("Part 1:\n\t{}", count);
}

fn solve_part2() {
    let lines = get_input();
    let mut grid = get_grid(&lines);

    for line in lines {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            for x in line.start.x.min(line.end.x)..=line.end.x.max(line.start.x) {
                let y = if line.end.y + line.end.x == line.start.y + line.start.x {
                    line.end.y + line.end.x - x
                } else {
                    line.start.y - line.start.x + x
                };

                grid[x as usize][y as usize] += 1;
            }
            continue;
        }

        for x in line.start.x.min(line.end.x)..=line.end.x.max(line.start.x) {
            for y in line.start.y.min(line.end.y)..=line.end.y.max(line.start.y) {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    let mut count = 0;
    for row in grid {
        for num in row {
            if num >= 2 {
                count += 1;
            }
        }
    }

    println!("Part 1:\n\t{}", count);
}
