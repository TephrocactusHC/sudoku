# sudoku
Generate sudoku problems &amp; solve sudoku problems

*for Software Engineering*
# TODO
- [x] Differents Ends
- [x] Input through command line parameters
- [x] Logo.
# Usage
You can use the following command:
```
cargo run -- -n <num> -m <num> -s
```
Among them, <num> is the number you want to specify for generating the quantity of problems and unknown numbers. The -s parameter indicates solving the problem. You can also use the -h parameter to display help information.


For example, if you want to generate 5 questions with 15 unknown numbers in each question and solve them, you can run the following command:
```
cargo run -- -n 5 -m 15 -s
```
The program will generate Sudoku puzzles and solve them based on the parameters you provide.

For more, you can use -h for help:
```
cargo run -- -h
```
# More
In fact, I haven't delved into the mathematical principles behind the Sudoku game, such as why there are only 36 final solutions in situations that are asymmetric and non-repetitive. Therefore, I don't know what problems generating a large number of final solutions will cause. This project is for entertainment purposes only. Due to the use of backtracking algorithms, performance may be very poor once the quantity becomes large.
