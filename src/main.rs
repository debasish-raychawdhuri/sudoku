mod naive_solver;
mod z3solver;
#[cfg(test)]
mod test_utils;

use naive_solver::solve_sudoku;

fn main() {
    println!("Enter the sudoku problem:");
    let mut input = String::new();
    let mut sudoku_problem = [[0; 9]; 9];
    for i in 0..9 {
        std::io::stdin().read_line(&mut input).unwrap();
        let line = input.trim();

        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                sudoku_problem[i][j] = 0;
                continue;
            }
            sudoku_problem[i][j] = c.to_digit(10).unwrap() as i32;
        }
        input.clear();
    }
    let solution = solve_sudoku(&sudoku_problem);
    print_sudoku(&solution);
}

fn print_sudoku(sudoku: &[Vec<i32>]) {
    println!("┌───────┬───────┬───────┐");
    for i in 0..9 {
        print!("│ ");
        for j in 0..9 {
            print!("{}", if sudoku[i][j] == 0 { ".".to_string() } else {  sudoku[i][j].to_string() });
            if (j + 1) % 3 == 0 {
                print!(" │ ");
            } else {
                print!(" ");
            }
        }
        println!();
        if (i + 1) % 3 == 0 && i < 8 {
            println!("├───────┼───────┼───────┤");
        }
    }
    println!("└───────┴───────┴───────┘");
}
