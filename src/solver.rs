//
// Puzzle solver
//

use crate::common::{ Data };
use crate::solver_15;

pub fn solve(puzzle_type: &str, input: &Data) {
    let result = match puzzle_type {
        "15" => solver_15::solve(input),
        _    => { println!("Unknown type: {}", puzzle_type); None }
    };
    println!("{:?}", result);
}

