
use std::fmt;

use itertools;

use super::common::{ Data };
//type Data = Vec<Vec<usize>>;

#[derive(Clone)]
pub struct Pattern {
    data: Data,
    n: usize,
    last_move: Move
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    direction: Direction,
    index: usize,
    distance: usize
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Horizontal,
    Vertical
}

impl Pattern {
    pub fn default(n: usize) -> Pattern {
        Pattern {
            data: (1..=n*n).collect(), 
            n: n,
            last_move: Move {
                direction: Direction::Horizontal,
                index: 0,
                distance: 0
            }
        }
    }

    pub fn from_input(input: &Data) -> Pattern {
        let n = input.len();
        let mut base = Pattern::default(n);
        for i in 0..n {
            base.data[i+n*n-n] = input[i];
        }
        base
    }

    pub fn apply_move(&self, m: &Move) -> Pattern {
        let mut result = self.clone();
        let n = self.n;
        match m.direction {
            Direction::Horizontal => {
                let shift_index = m.index * n;
                let shift_amount = n - m.distance;
                for i in 0..n {
                    result.data[shift_index+i]
                        = self.data[shift_index + (shift_amount+i) % n];
                }
            },
            Direction::Vertical => {
                let shift_index = m.index;
                let shift_amount = n - m.distance;
                for i in 0..n {
                    result.data[shift_index + i * n]
                        = self.data[shift_index + ((i+shift_amount) % n)*n];
                }
            }
        }

        result.last_move = m.clone();

        result
    }

    pub fn possible_patterns(&self) -> Vec<Pattern> {
        let n = self.n;
        let indices: Vec<usize> = (0..n).collect();
        let directions = [Direction::Horizontal, Direction::Vertical];
        let distances: Vec<usize> = (1..n).collect();

        let moves = itertools::iproduct!(
            directions.iter(), indices.iter(), distances.iter()
            )
            .map(|(&direction, &index, &distance)| Move { direction, index, distance })
            .filter(|m| *m != self.last_move);
        let patterns: Vec<Pattern> = moves.map(|m| self.apply_move(&m)).collect();

        //for p in &patterns {
        //    println!("{}", p);
        //}
        
        patterns
    }
}

impl Move {
    pub fn reverse(&self, n: usize) -> Move {
        Move {
            direction: self.direction,
            index: self.index,
            distance: (self.distance + n - 1) % n,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.n;
        for (i, e) in self.data.iter().enumerate() {
            write!(f, "{:02} ", e)?;
            if i % n == n-1 { write!(f, "\n")?; }
        }
        match self.last_move.direction {
            Direction::Horizontal => write!(f, "H,")?,
            Direction::Vertical   => write!(f, "V,")?
        };
        write!(f, "{},{}\n", self.last_move.index, self.last_move.distance)?;

        Ok(())
    }
}

struct PatternNode {
    pattern: Pattern,
    children: Vec<PatternNode>
}

pub fn solve(input: &Data) -> Option<Vec<Pattern>> {
    
    let target = Pattern::default(input.len());
    println!("target:\n{}", target);

    let start = Pattern::from_input(input);
    println!("start:\n{}", start);

    //let patterns = start.possible_patterns();
    
    let mut root = PatternNode { pattern: start, children: vec![] };
    let mut result: Vec<Pattern> = vec![];
    for depth in 0..(80/target.data.len()) {
        println!("depth: {}", depth);

        if search_and_build_tree(&mut root, &target.data, depth, &mut result) {
            println!("found!");
            return Some(result);
        }
    }

    None
}

fn search_and_build_tree(node: &mut PatternNode, target: &Data, depth: usize, result: &mut Vec<Pattern>) -> bool {
    if depth <= 0 {
        if node.pattern.data == *target {
            result.push(node.pattern.clone());
            return true;
        }
        return false;
    }

    if node.children.len() <= 0 {
        for p in node.pattern.possible_patterns() {
            node.children.push(PatternNode { pattern: p, children: vec![] });
        }
    }

    for p in &mut node.children {
        if search_and_build_tree(p, target, depth - 1, result) {
            result.push(node.pattern.clone());
            return true;
        }
    }

    false
}

