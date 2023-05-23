mod generate;
mod solve;
use std::env;

fn main() {
    print_text_art();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut num_puzzles = 10;
    let mut num_unknowns = 20;
    let mut solve_puzzles = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-n" => {
                if i + 1 < args.len() {
                    num_puzzles = parse_arg(&args[i + 1], num_puzzles);
                }
            }
            "-m" => {
                if i + 1 < args.len() {
                    num_unknowns = parse_arg(&args[i + 1], num_unknowns);
                }
            }
            "-s" => {
                solve_puzzles = true;
            }
            "-h" => {
                print_help();
                return;
            }
            _ => {}
        }
    }

    generate::generate_sudoku(num_puzzles, num_unknowns);

    if solve_puzzles {
        solve::solve_sudoku_puzzles();
    }
}

fn parse_arg(arg: &str, default: usize) -> usize {
    match arg.parse::<usize>() {
        Ok(value) => value,
        Err(_) => default,
    }
}

fn print_help() {
    println!("Usage: program_name [options]");
    println!("Options:");
    println!("-n <num>       Number of puzzles to generate (default: 10)");
    println!("-m <num>       Number of unknowns in each puzzle (default: 20)");
    println!("-s             Solve the generated puzzles");
    println!("-h             Display this help message");
}
        
fn print_text_art() {
    let art = "

░██████╗██╗░░░██╗██████╗░░█████╗░██╗░░██╗██╗░░░██╗
██╔════╝██║░░░██║██╔══██╗██╔══██╗██║░██╔╝██║░░░██║
╚█████╗░██║░░░██║██║░░██║██║░░██║█████═╝░██║░░░██║
░╚═══██╗██║░░░██║██║░░██║██║░░██║██╔═██╗░██║░░░██║
██████╔╝╚██████╔╝██████╔╝╚█████╔╝██║░╚██╗╚██████╔╝
╚═════╝░░╚═════╝░╚═════╝░░╚════╝░╚═╝░░╚═╝░╚═════╝░
        
";
         println!("{}", art);
}

