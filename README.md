# Sudoku Solver

A Rust implementation of sudoku solvers using two different approaches:
1. **Naive Backtracking Solver** - A recursive backtracking algorithm
2. **Z3 SMT Solver** - Using the Z3 theorem prover for constraint satisfaction

## Features

- Two different solving algorithms for comparison
- Comprehensive test suite with unit tests, integration tests, and benchmarks
- Support for standard sudoku file format (9x9 grid with '.' for empty cells)
- Performance benchmarking to compare solver efficiency

## Usage

### Running the Application
```bash
cargo run
```
Enter a 9x9 sudoku puzzle line by line, using '.' for empty cells.

### Running Tests
```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

## Test Suite

### Unit Tests
- **Naive Solver Tests** (`src/naive_solver.rs`):
  - `test_find_next_empty()` - Tests empty cell detection
  - `test_is_valid_move()` - Tests move validation logic
  - `test_solve_sudoku_*()` - Tests solving various difficulty levels
  - `test_solve_function_with_*_input()` - Tests error handling
  - `test_is_valid_initial_state()` - Tests initial sudoku validation
  - `test_solve_sudoku_with_invalid_input()` - Tests invalid input handling

- **Z3 Solver Tests** (`src/z3solver.rs`):
  - `test_add_constants()` - Tests Z3 variable creation
  - `test_add_external_constraints()` - Tests constraint generation
  - `test_solve_sudoku_*()` - Tests solving various difficulty levels
  - `test_exactly_one_constraint()` - Tests constraint logic

- **Test Utilities** (`src/test_utils.rs`):
  - `test_valid_sudoku_solution()` - Tests solution validation
  - `test_parse_sudoku_file()` - Tests file parsing
  - `test_solution_matches_problem()` - Tests constraint compliance

### Integration Tests (`tests/integration_tests.rs`)
- Tests both solvers with actual sudoku files (`test1.sudoku`, `test2.sudoku`, `test3.sudoku`)
- Verifies solver consistency and determinism
- Ensures original clues are preserved in solutions
- Validates that all cells are properly filled

### Benchmark Tests (`benches/sudoku_benchmark.rs`)
Performance comparison between naive and Z3 solvers:

#### Key Performance Results:
- **Easy Problems**: Naive solver ~1.3ms, Z3 solver ~22ms
- **Medium Problems**: Naive solver ~398ms, Z3 solver ~28ms  
- **Empty Sudoku**: Naive solver ~116μs, Z3 solver ~30ms
- **Z3 Setup Overhead**: ~1ms per solve

#### Performance Analysis:
- **Naive solver** is faster for easy problems and empty grids
- **Z3 solver** shows consistent performance regardless of difficulty
- **Z3 solver** excels at medium/hard problems where backtracking struggles
- **Z3 setup overhead** is significant for simple problems

## Test Files

The project includes three test sudoku files:
- `test1.sudoku` - Standard difficulty puzzle
- `test2.sudoku` - Alternative test case  
- `test3.sudoku` - Challenging puzzle

## Project Structure

```
src/
├── main.rs           # CLI application entry point
├── lib.rs            # Library interface
├── naive_solver.rs   # Backtracking algorithm implementation
├── z3solver.rs       # Z3 SMT solver implementation
└── test_utils.rs     # Testing utilities and validation functions

tests/
└── integration_tests.rs  # Integration tests with real sudoku files

benches/
└── sudoku_benchmark.rs   # Performance benchmarks

test*.sudoku          # Test sudoku puzzle files
```

## Dependencies

- `z3` (0.12.1) - Z3 SMT solver bindings
- `criterion` (0.5) - Benchmarking framework (dev dependency)

## Running Individual Test Categories

```bash
# Unit tests for naive solver
cargo test naive_solver::tests

# Unit tests for Z3 solver  
cargo test z3solver::tests

# Test utilities
cargo test test_utils::tests

# Integration tests
cargo test --test integration_tests

# Specific benchmark
cargo bench naive_solver_test1
```

## Important Bug Fix: Infinite Loop Prevention

### Problem
The naive solver previously had a critical bug where it would enter an infinite loop when given invalid initial sudoku states (e.g., duplicate numbers in the same row, column, or 3x3 box). This happened because:

1. The solver would try to fill empty cells without validating the initial state
2. Invalid initial constraints would make the puzzle unsolvable
3. The backtracking algorithm would loop infinitely trying to find a solution that doesn't exist

### Solution
Added [`is_valid_initial_state()`](src/naive_solver.rs:25) function that validates the initial sudoku before attempting to solve:

- **Checks rows**: No duplicate non-zero values in any row
- **Checks columns**: No duplicate non-zero values in any column
- **Checks 3x3 boxes**: No duplicate non-zero values in any 3x3 box

### Behavior Changes
- **Before**: [`solve_sudoku()`](src/naive_solver.rs:67) would panic or hang on invalid input
- **After**: Returns empty solution (all zeros) for invalid initial states
- **Performance**: Invalid inputs are rejected in O(1) time instead of infinite loops

### Test Coverage
- [`test_is_valid_initial_state()`](src/naive_solver.rs:275) - Tests validation logic
- [`test_solve_sudoku_with_invalid_input()`](src/naive_solver.rs:240) - Tests invalid input handling
- All tests now complete in under 30 seconds (previously could hang indefinitely)

## Test Coverage

The test suite provides comprehensive coverage:
- ✅ Algorithm correctness verification
- ✅ Edge case handling (empty sudoku, invalid input)
- ✅ **Infinite loop prevention** for invalid initial states
- ✅ Performance benchmarking
- ✅ Integration testing with real puzzle files
- ✅ Solution validation and constraint checking
- ✅ Solver consistency and determinism testing

## Contributing

When adding new features:
1. Add corresponding unit tests
2. Update integration tests if needed
3. Add benchmarks for performance-critical changes
4. Ensure all tests pass with `cargo test`
5. Run benchmarks to verify performance impact