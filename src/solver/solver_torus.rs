
use std::fmt;

use itertools;

//use super::common::{ Data };
type Data = Vec<usize>;

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
        let mut pattern = Pattern {
            data: vec![0; n],
            n: n,
            last_move: Move::default()
        };
        for i in 0..n {
            for j in 0..n {
                pattern.data[i] |= (i*n+j+1) << 8*j;
            }
        }

        pattern
    }

    pub fn from_input(input: &Data) -> Pattern {
        let n = input.len();
        let mut base = Pattern::default(n);
        base.data[n-1] = 0;
        for i in 0..n {
            base.data[n-1] |= input[i] << 8*i;
        }
        base
    }

    pub fn from_input_all(input: &Data) -> Pattern {
        let n = (input.len() as f32).sqrt() as usize;
        let mut data: Data = vec![0_usize; n];
        for irow in 0..n {
            for icolumn in 0..n {
                data[irow] |= input[irow*n + icolumn] << 8*icolumn;
            }
        }
        Pattern { data: data, n: n, last_move: Move::default() }
    }

    pub fn apply_move(&self, m: &Move) -> Pattern {
        let mut result = self.clone();
        let n = self.n;
        match m.direction {
            Direction::Horizontal => {
                let shift_index = m.index;
                let shift_amount = n - m.distance;
                result.data[shift_index]  = 0;
                result.data[shift_index] |= self.data[shift_index] << 8*shift_amount;
                result.data[shift_index] |= self.data[shift_index] >> (8*(n - shift_amount));
                result.data[shift_index] &= (2_i32.pow((n*8).try_into().unwrap()) - 1) as usize;
            },
            Direction::Vertical => {
                let shift_index = m.index;
                let shift_amount = n - m.distance;
                for i in 0..n {
                    result.data[i] &= !(0xff << shift_index * 8);
                }
                for i in 0..n {
                    result.data[i]
                        |= self.data[(i+shift_amount) % n] & (0xff << shift_index * 8);
                }
            }
        }

        result.last_move = m.clone();

        result
    }

    pub fn possible_patterns(&self) -> impl Iterator<Item = Pattern> + '_ {
        let n = self.n;
        let indices = 0..n;
        static DIRECTIONS: [Direction; 2] = [Direction::Horizontal, Direction::Vertical];
        let distances = 1..n;

        itertools::iproduct!(
            DIRECTIONS.iter(), indices, distances
            )
            .map(|(&direction, index, distance)| Move { direction, index, distance })
            .filter(|m| *m != self.last_move)
            .map(|m| self.apply_move(&m))
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

    pub fn default() -> Move {
        Move {
            direction: Direction::Horizontal,
            index: 0,
            distance: 0
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.n;
        for (i, e) in self.data.iter().enumerate() {
            for j in 0..n {
                write!(f, "{:02} ", (e >> 8*j)&0xff)?;
            }
            write!(f, "\n")?;
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

pub fn solve(input: &Vec<usize>) -> Option<Vec<Pattern>> {

    let (n, start) = if input.len() >= 9 {
        (
            (input.len() as f32).sqrt() as usize,
            Pattern::from_input_all(&input)
        )
    } else {
        ( input.len(), Pattern::from_input(&input) )
    };


    let target = Pattern::default(n);
    println!("target:\n{}", target);

    println!("start:\n{}", start);

    
    let mut root = PatternNode { pattern: start, children: vec![] };
    let mut result: Vec<Pattern> = vec![];
    for depth in 0..(120/target.data.len()) {
        println!("depth: {}", depth);

        if let Some(result) = search_and_build_tree(&mut root, &target.data, depth) {
            println!("found!");
            return Some(result);
        }
    }

    None
}

fn search_and_build_tree(node: &mut PatternNode, target: &Data, depth: usize) -> Option<Vec<Pattern>> {
    if depth <= 0 {
        if node.pattern.data == *target {
            return Some(vec![node.pattern.clone()]);
        }
        return None;
    }

    //if node.children.len() <= 0 {
    //    for p in node.pattern.possible_patterns() {
    //        node.children.push(PatternNode { pattern: p, children: vec![] });
    //    }
    //}
    node.pattern.possible_patterns()
        .map(|p| PatternNode { pattern: p, children: vec![] })
        .find_map(|mut p| match search_and_build_tree(&mut p, target, depth-1) {
            Some(mut v) => { v.push(p.pattern); Some(v) },
            None    => None
        })

    //for p in &mut node.children {
    //    if search_and_build_tree(p, target, depth - 1, result) {
    //        result.push(node.pattern.clone());
    //        return true;
    //    }
    //}
}

