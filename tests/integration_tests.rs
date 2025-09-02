//! Integration tests for the sudoku solvers using test files

use std::fs;
use sudoku::test_utils::*;

// Import both solvers
mod naive_solver {
    pub use sudoku::naive_solver::solve_sudoku;
}

mod z3solver {
    pub use sudoku::z3solver::solve_sudoku;
}

fn load_test_file(filename: &str) -> [[i32; 9]; 9] {
    let content = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read test file: {}", filename));
    parse_sudoku_file(&content)
}

#[test]
fn test_naive_solver_with_test1() {
    let problem = load_test_file("test1.sudoku");
    let solution = naive_solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_naive_solver_with_test2() {
    let problem = load_test_file("test2.sudoku");
    let solution = naive_solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_naive_solver_with_test3() {
    let problem = load_test_file("test3.sudoku");
    let solution = naive_solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_z3_solver_with_test1() {
    let problem = load_test_file("test1.sudoku");
    let solution = z3solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_z3_solver_with_test2() {
    let problem = load_test_file("test2.sudoku");
    let solution = z3solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_z3_solver_with_test3() {
    let problem = load_test_file("test3.sudoku");
    let solution = z3solver::solve_sudoku(&problem);
    
    assert!(is_valid_sudoku_solution(&solution), "Solution should be valid");
    assert!(solution_matches_problem(&problem, &solution), "Solution should match problem constraints");
}

#[test]
fn test_both_solvers_produce_same_results() {
    let test_files = ["test1.sudoku", "test2.sudoku", "test3.sudoku"];
    
    for test_file in &test_files {
        let problem = load_test_file(test_file);
        
        let naive_solution = naive_solver::solve_sudoku(&problem);
        let z3_solution = z3solver::solve_sudoku(&problem);
        
        // Both solutions should be valid
        assert!(is_valid_sudoku_solution(&naive_solution), 
                "Naive solution should be valid for {}", test_file);
        assert!(is_valid_sudoku_solution(&z3_solution), 
                "Z3 solution should be valid for {}", test_file);
        
        // Both should match the problem constraints
        assert!(solution_matches_problem(&problem, &naive_solution), 
                "Naive solution should match problem for {}", test_file);
        assert!(solution_matches_problem(&problem, &z3_solution), 
                "Z3 solution should match problem for {}", test_file);
        
        // Note: We don't assert that solutions are identical because
        // there might be multiple valid solutions to a sudoku puzzle
    }
}

#[test]
fn test_solver_consistency() {
    // Test that running the same solver multiple times gives the same result
    let problem = load_test_file("test1.sudoku");
    
    let solution1 = naive_solver::solve_sudoku(&problem);
    let solution2 = naive_solver::solve_sudoku(&problem);
    
    assert_eq!(solution1, solution2, "Naive solver should be deterministic");
    
    let z3_solution1 = z3solver::solve_sudoku(&problem);
    let z3_solution2 = z3solver::solve_sudoku(&problem);
    
    assert_eq!(z3_solution1, z3_solution2, "Z3 solver should be deterministic");
}

#[test]
fn test_empty_cells_are_filled() {
    let problem = load_test_file("test1.sudoku");
    let solution = naive_solver::solve_sudoku(&problem);
    
    // Ensure no cell is left empty (0)
    for i in 0..9 {
        for j in 0..9 {
            assert_ne!(solution[i][j], 0, "Cell ({}, {}) should not be empty", i, j);
            assert!(solution[i][j] >= 1 && solution[i][j] <= 9, 
                   "Cell ({}, {}) should contain a digit 1-9", i, j);
        }
    }
}

#[test]
fn test_original_clues_preserved() {
    let test_files = ["test1.sudoku", "test2.sudoku", "test3.sudoku"];
    
    for test_file in &test_files {
        let problem = load_test_file(test_file);
        let solution = naive_solver::solve_sudoku(&problem);
        
        // Check that all original clues are preserved
        for i in 0..9 {
            for j in 0..9 {
                if problem[i][j] != 0 {
                    assert_eq!(problem[i][j], solution[i][j], 
                              "Original clue at ({}, {}) should be preserved in {}", i, j, test_file);
                }
            }
        }
    }
}