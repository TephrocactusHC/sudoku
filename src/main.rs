mod generate;
mod solve;

fn main() {
    generate::generate_sudoku(10, 20); // Generate 10 puzzles with 20 unknowns each
    solve::solve_sudoku_puzzles();
}

