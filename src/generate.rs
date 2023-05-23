use std::fs::File;
use std::io::Write;
use rand::Rng;

pub fn generate_sudoku(num_puzzles: usize, num_unknowns: usize) {
    let mut rng = rand::thread_rng();

    for puzzle_num in 1..=num_puzzles {
        let mut puzzle = vec![vec![0; 9]; 9];

        // Fill in a complete Sudoku solution
        solve_sudoku(&mut puzzle);

        // Remove random numbers to create the puzzle
        for _ in 0..num_unknowns {
            let row = rng.gen_range(0, 9);
            let col = rng.gen_range(0, 9);

            puzzle[row][col] = 0;
        }

        // Write the puzzle to a text file
        let file_name = format!("puzzles/puzzle{}.txt", puzzle_num);
        let mut file = File::create(file_name).expect("Failed to create file");

        for row in &puzzle {
            let line: Vec<String> = row.iter().map(|&n| n.to_string()).collect();
            let line_str = line.join(" ");
            writeln!(file, "{}", line_str).expect("Failed to write to file");
        }
    }
}

fn solve_sudoku(puzzle: &mut Vec<Vec<u32>>) -> bool {
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

fn is_valid(puzzle: &Vec<Vec<u32>>, row: usize, col: usize, num: u32) -> bool {
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

