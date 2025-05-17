use aoc_util::direction::Direction;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct SearchState {
    pub position: (usize, usize),
    pub direction: Direction,
    pub score: usize,
}

impl SearchState {
    pub fn without_score(&self) -> ((usize, usize), Direction) {
        (self.position, self.direction)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
