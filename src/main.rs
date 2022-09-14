//
// Puzzle solver for expotential idle sub games
//

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Daiji Yamashita", 
    version = "0.1.0", 
    about = "Puzzle solver for expotential idle sub games",
)]
struct Args {
    #[clap(short = 't', long = "type", value_parser)]
    puzzle_type: String,
    #[clap(short = 'i', long = "input", value_parser, multiple = true)]
    input: Vec<usize>
}

mod puzzle;
mod solver;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    solver::solve(&args.puzzle_type, args.input);
}
