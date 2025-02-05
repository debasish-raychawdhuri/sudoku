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
