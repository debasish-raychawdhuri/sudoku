fn solve(sudoku_problem: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, &'static str> {
    let mut solution = sudoku_problem.to_vec();
    let next_empty = find_next_empty(&solution);
    if next_empty == (9, 9) {
        return Ok(solution);
    }
    let (row, col) = next_empty;
    for num in 1..=9 {
        if is_valid_move(&solution, row, col, num) {
            solution[row][col] = num;
            let result = solve(&solution);
            match result {
                Ok(sol) => return Ok(sol),
                Err(_) => solution[row][col] = 0,
            }
        }
    }
    return Err("No solution found");
}
pub fn solve_sudoku(sudoku_problem: &[[i32; 9]; 9]) -> Vec<Vec<i32>> {
    let sudoku_problem: Vec<Vec<i32>> = sudoku_problem.iter().map(|row| row.to_vec()).collect();
    solve(&sudoku_problem).unwrap()
}
fn find_next_empty(sudoku: &[Vec<i32>]) -> (usize, usize) {
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                return (i, j);
            }
        }
    }
    (9, 9)
}
fn is_valid_move(sudoku: &[Vec<i32>], row: usize, col: usize, num: i32) -> bool {
    for i in 0..9 {
        if sudoku[row][i] == num || sudoku[i][col] == num {
            return false;
        }
    }
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }
    true
}
