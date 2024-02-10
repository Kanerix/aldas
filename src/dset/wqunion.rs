use super::UnionFind;

pub struct WeightedQuickUnion {
    elements: Vec<usize>,
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind for WeightedQuickUnion {
    fn new(n: usize) -> Self {
        todo!()
    }

    fn extend(&mut self, n: usize) {
        todo!()
    }

    fn union(&mut self, p: usize, q: usize) {
        todo!()
    }

    fn find_leader(&self, p: usize) -> usize {
        todo!()
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        todo!()
    }

    fn move_to(&mut self, p: usize, q: usize) {
        // I am not clever enough to figure out how to
        // move an element and not fuck up its rank.
        unimplemented!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}
