//! Sudoku solver library with naive backtracking and Z3-based solvers

pub mod naive_solver;
pub mod z3solver;
pub mod test_utils;

// Re-export main functions for easier access
pub use naive_solver::solve_sudoku as naive_solve;
pub use z3solver::solve_sudoku as z3_solve;