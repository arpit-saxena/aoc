use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<Vec<i32>> {
    let file = File::open("inputs/2021/11.txt").expect("Expected input file of day 11 as 11.txt");
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn increase_level(level: &mut i32, num_to_flash: &mut i32) {
    *level = (level.abs() + 1) * level.signum();
    if *level > 9 {
        *num_to_flash += 1;
    }
}

// Performs a step on the given grid and returns number of flashes
// that happened in this step
fn do_step(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut num_flashes = 0;
    let mut num_to_flash = 0;
    for row in grid.iter_mut() {
        for level in row.iter_mut() {
            *level += 1;
            if *level > 9 {
                num_to_flash += 1;
            }
        }
    }

    while num_to_flash > 0 {
        let mut new_num_flash = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 {
                    grid[i][j] *= -1;
                    num_flashes += 1;
                    if i > 0 {
                        increase_level(&mut grid[i-1][j], &mut new_num_flash);
                        if j > 0 {
                            increase_level(&mut grid[i-1][j-1], &mut new_num_flash);
                        }
                        if j + 1 < grid[i].len() {
                            increase_level(&mut grid[i-1][j+1], &mut new_num_flash);
                        }
                    }

                    if j > 0 {
                        increase_level(&mut grid[i][j-1], &mut new_num_flash);
                    }
                    if j + 1 < grid[i].len() {
                        increase_level(&mut grid[i][j+1], &mut new_num_flash);
                    }

                    if i + 1 < grid.len() {
                        increase_level(&mut grid[i+1][j], &mut new_num_flash);
                        if j > 0 {
                            increase_level(&mut grid[i+1][j-1], &mut new_num_flash);
                        }
                        if j + 1 < grid[i].len() {
                            increase_level(&mut grid[i+1][j+1], &mut new_num_flash);
                        }
                    }
                }
            }
        }
        num_to_flash = new_num_flash;
    }

    for row in grid.iter_mut() {
        for level in row.iter_mut() {
            assert!(*level <= 9);
            if level.abs() > 9 {
                *level = 0;
            }
        }
    }

    num_flashes
}

fn solve_part1() {
    let mut grid = get_input();
    let num_steps = 100;
    let mut num_flashes = 0;

    for _ in 0..num_steps {
        num_flashes += do_step(&mut grid);
    }
    println!("Part 1:\n\t{}", num_flashes);
}

fn solve_part2() {
    let mut grid = get_input();
    let mut curr_step = 1;

    while do_step(&mut grid) < (grid.len() * grid[0].len()) as i32 {
        curr_step += 1;
    }

    println!("Part 2:\n\t{}", curr_step);
}
