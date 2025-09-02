//! Test utilities for sudoku validation and test data management

use std::collections::HashSet;

/// Validates if a sudoku solution is correct
pub fn is_valid_sudoku_solution(sudoku: &[Vec<i32>]) -> bool {
    if sudoku.len() != 9 || sudoku.iter().any(|row| row.len() != 9) {
        return false;
    }

    // Check all numbers are between 1-9
    for row in sudoku {
        for &cell in row {
            if cell < 1 || cell > 9 {
                return false;
            }
        }
    }

    // Check rows
    for row in sudoku {
        let mut seen = HashSet::new();
        for &cell in row {
            if !seen.insert(cell) {
                return false;
            }
        }
    }

    // Check columns
    for col in 0..9 {
        let mut seen = HashSet::new();
        for row in 0..9 {
            let cell = sudoku[row][col];
            if !seen.insert(cell) {
                return false;
            }
        }
    }

    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = HashSet::new();
            for row in box_row * 3..(box_row + 1) * 3 {
                for col in box_col * 3..(box_col + 1) * 3 {
                    let cell = sudoku[row][col];
                    if !seen.insert(cell) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

/// Checks if a solution matches the original problem constraints
pub fn solution_matches_problem(problem: &[[i32; 9]; 9], solution: &[Vec<i32>]) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if problem[i][j] != 0 && problem[i][j] != solution[i][j] {
                return false;
            }
        }
    }
    true
}

/// Parses a sudoku file format into a 2D array
pub fn parse_sudoku_file(content: &str) -> [[i32; 9]; 9] {
    let mut sudoku = [[0; 9]; 9];
    let lines: Vec<&str> = content.trim().lines().collect();
    
    for (i, line) in lines.iter().take(9).enumerate() {
        for (j, c) in line.chars().take(9).enumerate() {
            sudoku[i][j] = if c == '.' { 0 } else { c.to_digit(10).unwrap_or(0) as i32 };
        }
    }
    
    sudoku
}

/// Creates test sudoku problems for unit testing
pub fn get_test_problems() -> Vec<([[i32; 9]; 9], &'static str)> {
    vec![
        // Easy problem
        ([
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ], "easy"),
        
        // Medium problem
        ([
            [0, 0, 0, 6, 0, 0, 4, 0, 0],
            [7, 0, 0, 0, 0, 3, 6, 0, 0],
            [0, 0, 0, 0, 9, 1, 0, 8, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 5, 0, 1, 8, 0, 0, 0, 3],
            [0, 0, 0, 3, 0, 6, 0, 4, 5],
            [0, 4, 0, 2, 0, 0, 0, 6, 0],
            [9, 0, 3, 0, 0, 0, 0, 0, 0],
            [0, 2, 0, 0, 0, 0, 1, 0, 0],
        ], "medium"),
        
        // Empty sudoku (all zeros)
        ([[0; 9]; 9], "empty"),
    ]
}

/// Creates an invalid sudoku for testing error cases
pub fn get_invalid_sudoku() -> [[i32; 9]; 9] {
    [
        [1, 1, 0, 0, 0, 0, 0, 0, 0], // Invalid: two 1s in first row
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku_solution() {
        let valid_solution = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        assert!(is_valid_sudoku_solution(&valid_solution));
    }

    #[test]
    fn test_invalid_sudoku_solution() {
        let invalid_solution = vec![
            vec![1, 1, 4, 6, 7, 8, 9, 1, 2], // Duplicate 1s
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        assert!(!is_valid_sudoku_solution(&invalid_solution));
    }

    #[test]
    fn test_parse_sudoku_file() {
        let content = "53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79";
        let parsed = parse_sudoku_file(content);
        assert_eq!(parsed[0][0], 5);
        assert_eq!(parsed[0][1], 3);
        assert_eq!(parsed[0][2], 0);
        assert_eq!(parsed[8][8], 9);
    }

    #[test]
    fn test_solution_matches_problem() {
        let problem = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        
        let solution = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        
        assert!(solution_matches_problem(&problem, &solution));
    }
}