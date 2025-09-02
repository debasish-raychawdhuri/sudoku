//! Benchmark tests comparing naive solver vs Z3 solver performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use sudoku::test_utils::*;

// Import both solvers
use sudoku::naive_solver;
use sudoku::z3solver;

fn load_test_file(filename: &str) -> [[i32; 9]; 9] {
    let content = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read test file: {}", filename));
    parse_sudoku_file(&content)
}

fn benchmark_naive_solver(c: &mut Criterion) {
    let test1 = load_test_file("test1.sudoku");
    let test2 = load_test_file("test2.sudoku");
    let test3 = load_test_file("test3.sudoku");
    
    c.bench_function("naive_solver_test1", |b| {
        b.iter(|| naive_solver::solve_sudoku(black_box(&test1)))
    });
    
    c.bench_function("naive_solver_test2", |b| {
        b.iter(|| naive_solver::solve_sudoku(black_box(&test2)))
    });
    
    c.bench_function("naive_solver_test3", |b| {
        b.iter(|| naive_solver::solve_sudoku(black_box(&test3)))
    });
}

fn benchmark_z3_solver(c: &mut Criterion) {
    let test1 = load_test_file("test1.sudoku");
    let test2 = load_test_file("test2.sudoku");
    let test3 = load_test_file("test3.sudoku");
    
    c.bench_function("z3_solver_test1", |b| {
        b.iter(|| z3solver::solve_sudoku(black_box(&test1)))
    });
    
    c.bench_function("z3_solver_test2", |b| {
        b.iter(|| z3solver::solve_sudoku(black_box(&test2)))
    });
    
    c.bench_function("z3_solver_test3", |b| {
        b.iter(|| z3solver::solve_sudoku(black_box(&test3)))
    });
}

fn benchmark_solver_comparison(c: &mut Criterion) {
    let test1 = load_test_file("test1.sudoku");
    
    let mut group = c.benchmark_group("solver_comparison");
    
    group.bench_function("naive", |b| {
        b.iter(|| naive_solver::solve_sudoku(black_box(&test1)))
    });
    
    group.bench_function("z3", |b| {
        b.iter(|| z3solver::solve_sudoku(black_box(&test1)))
    });
    
    group.finish();
}

fn benchmark_test_problems(c: &mut Criterion) {
    let test_problems = get_test_problems();
    
    let mut group = c.benchmark_group("test_problems");
    
    for (i, (problem, difficulty)) in test_problems.iter().enumerate() {
        group.bench_function(format!("naive_{}", difficulty), |b| {
            b.iter(|| naive_solver::solve_sudoku(black_box(problem)))
        });
        
        group.bench_function(format!("z3_{}", difficulty), |b| {
            b.iter(|| z3solver::solve_sudoku(black_box(problem)))
        });
    }
    
    group.finish();
}

fn benchmark_empty_sudoku(c: &mut Criterion) {
    let empty_sudoku = [[0; 9]; 9];
    
    let mut group = c.benchmark_group("empty_sudoku");
    
    group.bench_function("naive_empty", |b| {
        b.iter(|| naive_solver::solve_sudoku(black_box(&empty_sudoku)))
    });
    
    group.bench_function("z3_empty", |b| {
        b.iter(|| z3solver::solve_sudoku(black_box(&empty_sudoku)))
    });
    
    group.finish();
}

fn benchmark_solver_initialization(c: &mut Criterion) {
    // Benchmark just the solver setup overhead
    c.bench_function("z3_solver_setup_overhead", |b| {
        b.iter(|| {
            // This measures the overhead of Z3 context creation
            let config = z3::Config::new();
            let context = z3::Context::new(&config);
            let _solver = z3::Solver::new(&context);
            black_box(());
        })
    });
}

criterion_group!(
    benches,
    benchmark_naive_solver,
    benchmark_z3_solver,
    benchmark_solver_comparison,
    benchmark_test_problems,
    benchmark_empty_sudoku,
    benchmark_solver_initialization
);

criterion_main!(benches);