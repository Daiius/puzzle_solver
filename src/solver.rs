//
// Puzzle solver
//

//
// from solver.rs, ./solver/* is visible.
// "mod common" means ./solver/common.rs
// "mod solver_15" means ./solver/solver_15.rs
// ...i guess
//
// "pub" enable re-export of internal sources
//
pub mod common;
pub mod solver_15;
pub mod solver_torous;

// ./solver/* is visible.
use common::{ Data };

pub fn solve(puzzle_type: &str, input: &Data) {
    match puzzle_type {
        "15" => call_solver_15(input),
        "t"  => call_solver_torous(input),
        _    => {
            println!("Unknown type: {}", puzzle_type);
        }
    };
}

fn call_solver_15(input: &Data) {
    let result = solver_15::solve(input);
    match result {
        Some(patterns) => {
            for p in &patterns {
                println!("{}", p);
            }
        },
        None => {
            println!("not found...");
        }
    }
}

fn call_solver_torous(input: &Data) {
    let result = solver_torous::solve(input);
    match result {
        Some(patterns) => {
            for p in &patterns {
                println!("{}", p);
            }
        },
        None => {
            println!("not found...");
        }
    }
}

