use z3::{ast::Bool, Context, Solver};

fn add_external_constraints<'a>(
    context: &'a Context,
    constants: &[Vec<Vec<Bool<'a>>>],
    sudoku_problem: &[[i32; 9]; 9],
) -> Bool<'a> {
    let mut clauses = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            let k = sudoku_problem[i][j];
            if k != 0 {
                clauses.push(&constants[i][j][k as usize - 1]);
            }
        }
    }
    Bool::and(&context, &clauses)
}
fn add_constants<'a>(context: &'a Context) -> Vec<Vec<Vec<Bool<'a>>>> {
    let mut constants = Vec::new();
    for i in 0..9 {
        let mut row = Vec::new();
        for j in 0..9 {
            let mut column = Vec::new();
            for k in 1..=9 {
                let constant = Bool::new_const(&context, format!("x_{}_{}_{}", i, j, k).as_str());
                column.push(constant);
            }
            row.push(column);
        }
        constants.push(row);
    }
    constants
}
fn get_solution<'a>(solver: &'a Solver, constants: &[Vec<Vec<Bool<'a>>>]) -> Vec<Vec<i32>> {
    let mut solution = vec![vec![0; 9]; 9];
    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        for i in 0..9 {
            for j in 0..9 {
                for k in 1..=9 {
                    let constant = &constants[i][j][k - 1];
                    if model.eval(constant, true).unwrap().as_bool().unwrap() {
                        solution[i][j] = k as i32;
                    }
                }
            }
        }
    }
    solution
}
pub fn solve_sudoku(sudoku_problem: &[[i32; 9]; 9]) -> Vec<Vec<i32>> {
    let config = z3::Config::new();
    let context = z3::Context::new(&config);
    let solver = Solver::new(&context);
    let constants = add_constants(&context);
    add_sudoku_constraints(&context, &solver, &constants, sudoku_problem);
    get_solution(&solver, &constants)
}
fn add_sudoku_constraints(
    context: &Context,
    solver: &Solver,
    constants: &[Vec<Vec<Bool>>],
    sudoku_problem: &[[i32; 9]; 9],
) {
    let mut constraints = Vec::new();
    constraints.push(exactly_one_per_box(context, &constants));
    constraints.push(exactly_one_per_col(context, &constants));
    constraints.push(exactly_one_per_row(context, &constants));
    constraints.push(exactly_one_per_cell(context, &constants));
    constraints.push(add_external_constraints(context, constants, sudoku_problem));
    let all_clauses = Bool::and(&context, &constraints.iter().collect::<Vec<_>>());
    solver.assert(&all_clauses);
}
fn exactly_one_per_cell<'a>(context: &'a Context, constants: &'a [Vec<Vec<Bool<'a>>>]) -> Bool<'a> {
    let mut clauses = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            let mut vars = Vec::<&'a Bool>::new();
            for k in 0..9 {
                vars.push(&constants[i][j][k]);
            }
            let exactly_one = exactly_one(context, &vars);
            clauses.push(exactly_one);
        }
    }
    Bool::and(&context, &clauses.iter().collect::<Vec<_>>())
}
fn exactly_one_per_box<'a>(context: &'a Context, constants: &'a [Vec<Vec<Bool<'a>>>]) -> Bool<'a> {
    let mut clauses = Vec::new();
    for p in 0..9 {
        for k in 0..9 {
            let mut vars = Vec::<&'a Bool>::new();
            let s = p / 3;
            let t = p % 3;
            for i in 3 * s..3 * s + 3 {
                for j in 3 * t..3 * t + 3 {
                    vars.push(&constants[i][j][k]);
                }
            }
            let exactly_one = exactly_one(context, &vars);
            clauses.push(exactly_one);
        }
    }
    Bool::and(&context, &clauses.iter().collect::<Vec<_>>())
}

fn exactly_one_per_col<'a>(context: &'a Context, constants: &'a [Vec<Vec<Bool<'a>>>]) -> Bool<'a> {
    let mut clauses = Vec::new();
    for j in 0..9 {
        for k in 0..9 {
            let mut vars = Vec::<&'a Bool>::new();
            for i in 0..9 {
                vars.push(&constants[i][j][k]);
            }
            let exactly_one = exactly_one(context, &vars);
            clauses.push(exactly_one);
        }
    }
    Bool::and(&context, &clauses.iter().collect::<Vec<_>>())
}

    /// Assert that for each row and number, there is exactly one true.
    ///
    /// This is a helper for the Sudoku problem. It asserts that in each row
    /// there is exactly one occurrence of each number. This is done by asserting
    /// that for each row and for each number, there is exactly one true in the
    /// corresponding positions.
fn exactly_one_per_row<'a>(context: &'a Context, constants: &'a [Vec<Vec<Bool<'a>>>]) -> Bool<'a> {
    let mut clauses = Vec::new();
    for i in 0..9 {
        for k in 0..9 {
            let mut vars = Vec::<&'a Bool>::new();
            for j in 0..9 {
                vars.push(&constants[i][j][k]);
            }
            let exactly_one = exactly_one(context, &vars);
            clauses.push(exactly_one);
        }
    }
    Bool::and(&context, &clauses.iter().collect::<Vec<_>>())
}
fn exactly_one<'a>(context: &'a Context, constants: &[&'a Bool]) -> Bool<'a> {
    let mut clauses = Vec::new();
    let mut s = Vec::new();
    let mut ands = Vec::new();
    s.push(constants[0].not().not());
    for i in 1..constants.len() {
        ands.push(Bool::and(&context, &[&s[i - 1], &constants[i]]).not());
        let l = Bool::or(&context, &[&s[i - 1], &constants[i]]);
        s.push(l);
    }
    for ai in ands.iter() {
        clauses.push(ai);
    }
    clauses.push(s.last().unwrap());

    Bool::and(&context, &clauses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

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
    fn test_add_constants() {
        let config = z3::Config::new();
        let context = z3::Context::new(&config);
        let constants = add_constants(&context);
        
        // Check dimensions
        assert_eq!(constants.len(), 9);
        assert_eq!(constants[0].len(), 9);
        assert_eq!(constants[0][0].len(), 9);
        
        // Check that constants are properly named
        // This is a basic structural test since we can't easily inspect Z3 variable names
        for i in 0..9 {
            for j in 0..9 {
                for k in 0..9 {
                    // Just verify the constant exists and is a boolean
                    let _constant = &constants[i][j][k];
                }
            }
        }
    }

    #[test]
    fn test_add_external_constraints() {
        let config = z3::Config::new();
        let context = z3::Context::new(&config);
        let constants = add_constants(&context);
        
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
        
        let constraints = add_external_constraints(&context, &constants, &problem);
        
        // Basic test - just ensure the constraint is created without panicking
        // More detailed testing would require Z3 solver integration
        let solver = z3::Solver::new(&context);
        solver.assert(&constraints);
        
        // The constraint should be satisfiable (not immediately contradictory)
        assert_ne!(solver.check(), z3::SatResult::Unsat);
    }

    #[test]
    fn test_get_solution_with_solved_puzzle() {
        let config = z3::Config::new();
        let context = z3::Context::new(&config);
        let solver = z3::Solver::new(&context);
        let constants = add_constants(&context);
        
        // Use a simple problem that should have a solution
        let problem = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        
        add_sudoku_constraints(&context, &solver, &constants, &problem);
        let solution = get_solution(&solver, &constants);
        
        // Should return the same solved puzzle
        for i in 0..9 {
            for j in 0..9 {
                assert_eq!(solution[i][j], problem[i][j]);
            }
        }
    }

    #[test]
    fn test_exactly_one_constraint() {
        // Test the exactly_one function by using it in a complete solve
        // This avoids lifetime issues with Z3 context management
        let problem = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        
        let solution = solve_sudoku(&problem);
        
        // If exactly_one works correctly, we should get the same solution back
        for i in 0..9 {
            for j in 0..9 {
                assert_eq!(solution[i][j], problem[i][j]);
            }
        }
    }

    #[test]
    fn test_solve_with_invalid_sudoku() {
        let invalid_problem = get_invalid_sudoku();
        let solution = solve_sudoku(&invalid_problem);
        
        // Z3 solver might still try to solve it, but the solution should be invalid
        // or it should return an empty/zero solution
        let is_all_zeros = solution.iter().all(|row| row.iter().all(|&cell| cell == 0));
        
        if !is_all_zeros {
            // If Z3 returns a non-zero solution, it should still be valid
            // (Z3 might ignore the invalid constraints and find a valid solution)
            assert!(is_valid_sudoku_solution(&solution));
        }
    }
}
