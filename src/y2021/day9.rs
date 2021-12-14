use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn get_input() -> Vec<Vec<u32>> {
    let file = File::open("inputs/2021/9.txt").expect("Input file of day 9 expected as 9.txt");
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    grid
}

fn solve_part1() {
    let mut sum_risk = 0;
    let grid = get_input();
    for (i, row) in grid.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if (i > 0 && grid[i - 1][j] <= *height)
                || (i + 1 < grid.len() && grid[i + 1][j] <= *height)
                || (j > 0 && grid[i][j - 1] <= *height)
                || (j + 1 < grid[0].len() && grid[i][j + 1] <= *height)
            {
                continue;
            }

            sum_risk += height + 1;
        }
    }

    println!("Part 1:\n\t{}", sum_risk);
}

fn solve_part2() {
    let grid = get_input();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    fn dfs(i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
        if visited[i][j] {
            return 0;
        };
        visited[i][j] = true;
        let mut count = 1;
        if i > 0 {
            count += dfs(i - 1, j, visited);
        }
        if i + 1 < visited.len() {
            count += dfs(i + 1, j, visited);
        }
        if j > 0 {
            count += dfs(i, j - 1, visited);
        }
        if j + 1 < visited[0].len() {
            count += dfs(i, j + 1, visited);
        }

        count
    };

    for (i, row) in grid.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if num == 9 {
                visited[i][j] = true;
            }
        }
    }

    let mut heap = BinaryHeap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let count = dfs(i, j, &mut visited);
            if heap.len() < 3 {
                heap.push(Reverse(count));
            } else if heap.peek().unwrap().0 < count {
                heap.pop();
                heap.push(Reverse(count));
            }
        }
    }

    println!(
        "Part 2:\n\t{}",
        heap.pop().unwrap().0 * heap.pop().unwrap().0 * heap.pop().unwrap().0
    );
}
