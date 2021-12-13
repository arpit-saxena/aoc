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

struct Paper {
    grid : Vec<Vec<bool>>
}

impl Paper {
    fn new(x_size: i32, y_size: i32) -> Paper {
        Paper { grid : vec![vec![false; y_size as usize]; x_size as usize] }
    }

    fn mark(self : &mut Paper, x: i32, y: i32) {
        self.grid[x as usize][y as usize] = true;
    }

    fn fold_x(self : &mut Paper, fold_axis : usize) {
        let start = 0.max(2*fold_axis as i32 - self.grid.len() as i32 + 1);
        for x in start as usize..fold_axis {
            for y in 0..self.grid[0].len() {
                self.grid[x][y] |= self.grid[2*fold_axis-x][y];
            }
        }
        self.grid.truncate(fold_axis);
    }

    fn fold_y(self : &mut Paper, fold_axis : usize) {
        let start = 0.max(2*fold_axis as i32 - self.grid[0].len() as i32 + 1);
        for x in 0..self.grid.len() {
            for y in start as usize..fold_axis {
                self.grid[x][y] |= self.grid[x][2*fold_axis-y];
            }
        }

        self.grid.iter_mut().for_each(|v| v.truncate(fold_axis));
    }

    fn num_dots(self : &Paper) -> i32 {
        self.grid.iter().map(|row| row.iter().filter(|&&x| x).count() as i32).sum()
    }

    fn print(self : &Paper) {
        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                if self.grid[x][y] {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }
}

fn get_input() -> (Paper, Vec<(char, i32)>) {
    let file = File::open("inputs/2021/13.txt").expect("Expected input file for day 13 as 13.txt");
    let reader = BufReader::new(file);

    let mut iter = reader.lines().map(|l| l.unwrap());
    let mut points = Vec::<Point>::new();

    while let Some(line) = iter.next() {
        if line.len() == 0 {
            break;
        }

        let (x, y): (i32, i32);
        scan!(line.bytes() => "{},{}", x, y);
        points.push(Point { x, y });
    }

    let mut folds = Vec::<(char, i32)>::new();
    while let Some(line) = iter.next() {
        let (fold_dir, axis) : (char, i32);
        scan!(line.bytes() => "fold along {}={}", fold_dir, axis);
        folds.push((fold_dir, axis));
    }

    let (max_x, max_y) = points.iter().fold((0, 0), |(mut max_x, mut max_y), Point{x, y}| {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
        (max_x, max_y)
    });

    let size_x = if max_x % 2 == 0 { max_x + 1 } else { max_x + 2 };
    let size_y = if max_y % 2 == 0 { max_y + 1 } else { max_y + 2 };
    let mut paper = Paper::new(size_x, size_y);
    for Point{x, y} in points {
        paper.mark(x, y);
    }

    (paper, folds)
}

fn solve_part1() {
    let (mut paper, folds) = get_input();

    let (dir, axis) = folds[0];
    if dir == 'x' {
        paper.fold_x(axis as usize);
    } else {
        paper.fold_y(axis as usize);
    }

    println!("Part 1:\n\t{}", paper.num_dots());
}

fn solve_part2() {
    let (mut paper, folds) = get_input();
    
    for (dir, axis) in folds {
        if dir == 'x' {
            paper.fold_x(axis as usize);
        } else {
            paper.fold_y(axis as usize);
        }
    }

    print!("Part 2:\n\n");
    paper.print();
}