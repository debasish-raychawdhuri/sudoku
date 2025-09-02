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

/// Validates that the initial sudoku state is valid (no conflicts in given clues)
fn is_valid_initial_state(sudoku: &[Vec<i32>]) -> bool {
    // Check rows
    for row in 0..9 {
        let mut seen = std::collections::HashSet::new();
        for col in 0..9 {
            let val = sudoku[row][col];
            if val != 0 {
                if !seen.insert(val) {
                    return false; // Duplicate in row
                }
            }
        }
    }
    
    // Check columns
    for col in 0..9 {
        let mut seen = std::collections::HashSet::new();
        for row in 0..9 {
            let val = sudoku[row][col];
            if val != 0 {
                if !seen.insert(val) {
                    return false; // Duplicate in column
                }
            }
        }
    }
    
    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = std::collections::HashSet::new();
            for row in box_row * 3..(box_row + 1) * 3 {
                for col in box_col * 3..(box_col + 1) * 3 {
                    let val = sudoku[row][col];
                    if val != 0 {
                        if !seen.insert(val) {
                            return false; // Duplicate in box
                        }
                    }
                }
            }
        }
    }
    
    true
}

pub fn solve_sudoku(sudoku_problem: &[[i32; 9]; 9]) -> Vec<Vec<i32>> {
    let sudoku_problem: Vec<Vec<i32>> = sudoku_problem.iter().map(|row| row.to_vec()).collect();
    
    // Validate initial state first
    if !is_valid_initial_state(&sudoku_problem) {
        // Return empty solution for invalid input instead of panicking
        return vec![vec![0; 9]; 9];
    }
    
    match solve(&sudoku_problem) {
        Ok(solution) => solution,
        Err(_) => vec![vec![0; 9]; 9], // Return empty solution if no solution found
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_find_next_empty() {
        let sudoku = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(find_next_empty(&sudoku), (1, 0));

        let full_sudoku = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 5, 6, 7, 8, 9, 1, 2, 3],
            vec![7, 8, 9, 1, 2, 3, 4, 5, 6],
            vec![2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![5, 6, 7, 8, 9, 1, 2, 3, 4],
            vec![8, 9, 1, 2, 3, 4, 5, 6, 7],
            vec![3, 4, 5, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 8, 9, 1, 2, 3, 4, 5],
            vec![9, 1, 2, 3, 4, 5, 6, 7, 8],
        ];
        assert_eq!(find_next_empty(&full_sudoku), (9, 9));
    }

    #[test]
    fn test_is_valid_move() {
        let sudoku = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        // Valid moves
        assert!(is_valid_move(&sudoku, 0, 2, 4)); // Can place 4 at (0,2)
        assert!(is_valid_move(&sudoku, 1, 1, 7)); // Can place 7 at (1,1)

        // Invalid moves - number already in row
        assert!(!is_valid_move(&sudoku, 0, 2, 5)); // 5 already in row 0
        assert!(!is_valid_move(&sudoku, 0, 2, 3)); // 3 already in row 0

        // Invalid moves - number already in column
        assert!(!is_valid_move(&sudoku, 1, 1, 3)); // 3 already in column 1
        assert!(!is_valid_move(&sudoku, 1, 1, 9)); // 9 already in column 1

        // Invalid moves - number already in 3x3 box
        assert!(!is_valid_move(&sudoku, 0, 2, 6)); // 6 already in top-left box
        assert!(!is_valid_move(&sudoku, 2, 0, 5)); // 5 already in top-left box
    }

    #[test]
    fn test_solve_sudoku_easy() {
        let (problem, _) = &get_test_problems()[0]; // Easy problem
        let solution = solve_sudoku(problem);
        
        assert!(is_valid_sudoku_solution(&solution));
        assert!(solution_matches_problem(problem, &solution));
    }

    #[test]
    fn test_solve_sudoku_medium() {
        let (problem, _) = &get_test_problems()[1]; // Medium problem
        let solution = solve_sudoku(problem);
        
        assert!(is_valid_sudoku_solution(&solution));
        assert!(solution_matches_problem(problem, &solution));
    }

    #[test]
    fn test_solve_empty_sudoku() {
        let (problem, _) = &get_test_problems()[2]; // Empty sudoku
        let solution = solve_sudoku(problem);
        
        assert!(is_valid_sudoku_solution(&solution));
        assert!(solution_matches_problem(problem, &solution));
    }

    #[test]
    fn test_solve_function_with_valid_input() {
        let problem = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        
        let result = solve(&problem);
        assert!(result.is_ok());
        
        let solution = result.unwrap();
        assert!(is_valid_sudoku_solution(&solution));
    }

    #[test]
    fn test_solve_function_with_invalid_input() {
        let invalid_problem = vec![
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0], // Two 1s in first row
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        
        // First validate that the initial state is invalid
        assert!(!is_valid_initial_state(&invalid_problem));
        
        // The internal solve function should still return an error for invalid states
        // but we need to be careful not to call it on invalid initial states
        // Instead, let's test with a valid initial state that has no solution
        let unsolvable_problem = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 9], // This makes it unsolvable
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        
        let result = solve(&unsolvable_problem);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No solution found");
    }

    #[test]
    fn test_solve_sudoku_with_invalid_input() {
        let invalid_problem = [
            [1, 1, 0, 0, 0, 0, 0, 0, 0], // Two 1s in first row
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        
        // This should now return quickly with an empty solution instead of hanging
        let solution = solve_sudoku(&invalid_problem);
        
        // Should return empty solution (all zeros) for invalid input
        let expected_empty = vec![vec![0; 9]; 9];
        assert_eq!(solution, expected_empty);
    }

    #[test]
    fn test_is_valid_initial_state() {
        // Valid initial state
        let valid_problem = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(is_valid_initial_state(&valid_problem));

        // Invalid initial state - duplicate in row
        let invalid_row = vec![
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0], // Two 1s in first row
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(!is_valid_initial_state(&invalid_row));

        // Invalid initial state - duplicate in column
        let invalid_col = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0], // Two 1s in first column
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(!is_valid_initial_state(&invalid_col));

        // Invalid initial state - duplicate in 3x3 box
        let invalid_box = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0], // Two 1s in top-left box
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(!is_valid_initial_state(&invalid_box));
    }
}
