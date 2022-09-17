//
// 
//

use crate::common::{ Data, Num };

#[derive(Debug, Clone)]
pub struct Pattern {
    data: Data,
    last_move: Move
}

impl Pattern {
    pub fn default(n: Num) -> Pattern {
        let mut numbers: Data = ((n*n-2*n+1)..(n*n)).collect();
        numbers.push(0);
        Pattern { data: numbers, last_move: Move { from: 2*n-1, to: 2*n-1 } }
    }

    pub fn from_input(input: &Data) -> Pattern {
        let pos = input.iter().find(|&&n| n == 0).unwrap();
        Pattern { data: input.to_vec(), last_move: Move { from: *pos, to: *pos } }
    }

    pub fn possible_patterns(&self) -> Vec<Pattern> {
        let pos = self.last_move.to;
        let n = self.data.len() / 2;
        let x = pos % n;
        let y = pos / n;
        let mut patterns: Vec<Pattern> = vec![];
        
        // up
        if y > 0 {
            let up_pos = pos - n;
            if self.last_move.from != up_pos {
                let mut d = self.data.clone();
                d.swap(up_pos, pos);
                patterns.push(Pattern { data: d, last_move: Move { from: pos, to: up_pos }});
            }
        } 
        // down
        if y < 1 {
            let down_pos = pos + n;
            if self.last_move.from != down_pos {
                let mut d = self.data.clone();
                d.swap(down_pos, pos);
                patterns.push(Pattern { data: d, last_move: Move { from: pos, to: down_pos}});
            }
        }
        // left
        if x > 0 {
            let left_pos = pos - 1;
            if self.last_move.from != left_pos {
                let mut d = self.data.clone();
                d.swap(left_pos, pos);
                patterns.push(Pattern { data: d, last_move: Move { from: pos, to: left_pos}});
            }
        }
        if x < n-1 {
            let right_pos = pos + 1;
            if self.last_move.from != right_pos {
                let mut d = self.data.clone();
                d.swap(right_pos, pos);
                patterns.push(Pattern { data: d, last_move: Move { from: pos, to: right_pos}});
            }
        }
            
        patterns
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    from: Num,
    to: Num
}

impl Move {
    pub fn default() -> Move {
        Move { from: 0, to: 0 }
    }

    pub fn reverse(&self) -> Move {
        Move { from: self.to, to: self.from }
    }
}

struct PatternNode {
    pattern: Pattern,
    children: Vec<PatternNode>
}

pub fn solve(input: &Data) -> Option<Vec<Pattern>> {
    let target = Pattern::default(input.len() / 2);
    let mut result: Vec<Pattern> = vec![];
    let mut root = PatternNode { pattern: target.clone(), children: vec![]};
    println!("{:?}", target);

    let max_depth = 35;
    for depth in 0..max_depth {
        println!("depth: {}", depth);
        if search_and_extend_tree(&mut root, input, depth, &mut result) {

            return Some(result);
        }
    }

    None
}

fn search_and_extend_tree(
    node: &mut PatternNode,
    target: &Data,
    depth: usize,
    result: &mut Vec<Pattern>
) -> bool {
    
    // Check deepest node
    if depth <= 0 {
        if node.pattern.data == *target {
            println!("found!");
            result.push(node.pattern.clone());
            return true;
        }
        return false;
    }

    // Extend tree if needed
    if node.children.len() == 0 {
        let patterns = node.pattern.possible_patterns();
        for p in patterns {
            node.children.push(PatternNode { pattern: p, children: vec![] });
        }
    }

    for child in &mut node.children {
        if search_and_extend_tree(child, target, depth - 1, result) {
            result.push(node.pattern.clone());
            return true;
        }
    }
    return false;
}

