//
// Puzzle solver
//
// Puzzle: a problem this program solves.
//         3 kinds of puzzles are possible:
//         15-puzzle, slide puzzle, arrow puzzle
//     Pattern: sequence of numbers consists a puzzle.
//              (directions of arrows can be expressed by numbers, too)
//              in this program, many patterns are generated
//              and evaluated to search the target pattern.
//
//              for simplicity, this program usually solves limited part of the whole puzzle.
//              15-puzzle   : last two rows
//              slide puzzle: last one rows
//              arrow puzzle: unfortunately the whole puzzle
//              
//      Move: difference between current pattern and next pattern.
//            this  is useful to avoid searching unnecessary patterns,
//            e.g. undoing last move.
// PatternNode: 

use crate::puzzle::{ Move, Data, PatternNode };

pub fn solve(puzzle_type: &str, input: Data) {
    let mut root = PatternNode { pattern: input, children: vec![] };


}

fn search_patterns(node: &mut PatternNode, target: Data, depth: usize) -> Option<Vec<Move>> {

}
