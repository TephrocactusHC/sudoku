use std::fs::File;
use std::io::Write;
use rand::Rng;
use crate::solve::is_valid;


pub fn generate_sudoku(num_puzzles: usize, num_unknowns: usize) {
    let mut rng = rand::thread_rng();

    for puzzle_num in 1..=num_puzzles {
        let mut puzzle = vec![vec![0; 9]; 9];

        // Fill in a complete Sudoku solution
        solve_sudoku(&mut puzzle);

        // Randomly shuffle the rows and columns
        shuffle_rows(&mut puzzle, &mut rng);
        shuffle_columns(&mut puzzle, &mut rng);

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


fn shuffle_rows(puzzle: &mut Vec<Vec<u32>>, rng: &mut impl Rng) {
    for _ in 0..3 {
        let block_start = rng.gen_range(0, 3) * 3;
        let block_end = block_start + 3;
        let new_block_start = rng.gen_range(0, 3) * 3;

        for i in block_start..block_end {
            let new_i = new_block_start + (i - block_start);
            puzzle.swap(i, new_i);
        }
    }
}

fn shuffle_columns(puzzle: &mut Vec<Vec<u32>>, rng: &mut impl Rng) {
    transpose(puzzle); // Transpose the puzzle
    shuffle_rows(puzzle, rng); // Shuffle the rows
    transpose(puzzle); // Transpose it back to the original orientation
}

fn transpose(puzzle: &mut Vec<Vec<u32>>) {
    for i in 0..9 {
        for j in (i + 1)..9 {
            puzzle.swap(i, j);
        }
    }
}

