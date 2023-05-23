use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn solve_sudoku(puzzle: &mut Vec<Vec<u32>>) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(puzzle, row, col, num) {
                        puzzle[row][col] = num;
                        if solve_sudoku(puzzle) {
                            return true;
                        }
                        puzzle[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

pub fn is_valid(puzzle: &Vec<Vec<u32>>, row: usize, col: usize, num: u32) -> bool {
    for i in 0..9 {
        if puzzle[row][i] == num {
            return false;
        }
        if puzzle[i][col] == num {
            return false;
        }
        let box_row = 3 * (row / 3) + i / 3;
        let box_col = 3 * (col / 3) + i % 3;
        if puzzle[box_row][box_col] == num {
            return false;
        }
    }
    true
}

pub fn display_solution(puzzle: &Vec<Vec<u32>>) {
    for row in puzzle {
        let line: Vec<String> = row.iter().map(|&n| n.to_string()).collect();
        let line_str = line.join(" ");
        println!("{}", line_str);
    }
    println!();
}

pub fn solve_sudoku_puzzles() {
    let puzzle_dir = "./puzzles"; // 修改为数独问题文件夹的绝对路径

    let puzzle_files = fs::read_dir(puzzle_dir)
        .expect("Failed to read puzzle directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read puzzle file");
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().to_string();
            if file_name_str.ends_with(".txt") {
                Some(entry.path())
            } else {
                None
            }
        });

    for puzzle_file in puzzle_files {
        println!("Processing puzzle file: {:?}", puzzle_file);

        let file = File::open(puzzle_file).expect("Failed to open puzzle file");
        let reader = BufReader::new(file);
        let mut puzzle: Vec<Vec<u32>> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read puzzle file");
            let row: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse().expect("Invalid number in puzzle file"))
                .collect();
            puzzle.push(row);
        }

        println!("Original puzzle:");
        display_solution(&puzzle);

        if solve_sudoku(&mut puzzle) {
            println!("Solution:");
            display_solution(&puzzle);
        } else {
            println!("No solution found for this puzzle.");
        }
    }
}


