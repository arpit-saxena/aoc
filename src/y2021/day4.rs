use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    solve_part1();
    solve_part2();
}

struct Board {
    arr: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new() -> Board {
        Board {
            arr: [[0; 5]; 5],
            marked: [[false; 5]; 5],
        }
    }

    fn mark(self: &mut Board, num: i32) -> bool {
        for (row, marked_row) in self.arr.iter().zip(self.marked.iter_mut()) {
            for (num_board, marker) in row.iter().zip(marked_row.iter_mut()) {
                if *num_board == num {
                    *marker = true;
                }
            }
        }

        self.is_winning()
    }

    fn is_winning(self: &Board) -> bool {
        for row in &self.marked {
            if row.iter().all(|&x| x) {
                return true;
            }
        }

        for y in 0..5 {
            let mut all_marked = true;
            for x in 0..5 {
                all_marked = all_marked && self.marked[x][y];
            }

            if all_marked {
                return true;
            }
        }

        false
    }

    fn get_sum_unmarked(self: &Board) -> i32 {
        let mut sum = 0;
        for (row, marked_row) in self.arr.iter().zip(self.marked.iter()) {
            for (num, marker) in row.iter().zip(marked_row.iter()) {
                if !*marker {
                    sum += num;
                }
            }
        }
        sum
    }
}

fn get_input() -> (Vec<i32>, Vec<Board>) {
    let file = File::open("inputs/2021/4.txt").expect("Expected input file for day 4 as 3.txt");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let draw_order = line
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let board_nums = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut boards = Vec::<Board>::new();
    for board_arr in board_nums.chunks(25) {
        let mut board = Board::new();
        for idx in 0..25 {
            board.arr[idx / 5][idx % 5] = board_arr[idx];
        }
        boards.push(board);
    }

    (draw_order, boards)
}

fn solve_part1() {
    let (draw_order, mut boards) = get_input();

    for num in draw_order {
        for board in &mut boards {
            if board.mark(num) {
                println!("Part 1:\n\t{}", num * board.get_sum_unmarked());
                return;
            }
        }
    }
}

fn solve_part2() {
    let (draw_order, mut boards) = get_input();
    let num_boards = boards.len();
    let mut won = vec![false; boards.len()];
    let mut num_won = 0;

    for num in draw_order {
        for (board, has_won) in boards.iter_mut().zip(won.iter_mut()) {
            if *has_won {
                continue;
            }

            if board.mark(num) {
                num_won += 1;
                *has_won = true;

                if num_won == num_boards {
                    println!("Part 2:\n\t{}", num * board.get_sum_unmarked());
                }
            }
        }
    }
}
