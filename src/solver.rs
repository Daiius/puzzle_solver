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

// ./solver/* is visible.
use common::{ Data };

pub fn solve(puzzle_type: &str, input: &Data) {
    let result = match puzzle_type {
        "15" => solver_15::solve(input),
        _    => { println!("Unknown type: {}", puzzle_type); None }
    };
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

