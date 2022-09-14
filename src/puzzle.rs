
pub type Data = Vec<usize>;

pub struct PatternNode {
    pattern: Pattern,
    children: Vec<PatternNode>
}

pub trait Pattern {
    fn possible_patterns(&self) -> Vec<Move>;
}

pub trait Move {
    fn inverse(&self) -> Move;
}
