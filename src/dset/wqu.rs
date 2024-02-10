use super::UnionFind;

#[derive(Debug)]
pub struct WeightedQuickUnion {
    elements: Vec<usize>,
    parents: Vec<usize>,
    ranks: Vec<usize>,
    len: usize,
}

impl UnionFind for WeightedQuickUnion {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut ranks = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            parents.push(i);
            ranks.push(0);
        }

        WeightedQuickUnion {
            elements,
            parents,
            ranks,
            len: n,
        }
    }

    fn extend(&mut self, n: usize) {
        let count = self.count();
        let max = if count != 0 {
            self.elements[count - 1]
        } else {
            0
        };

        for i in max..max + n {
            self.elements.push(i);
            self.parents.push(i);
            self.ranks.push(i);
        }

        self.len += n - 1;
    }

    fn clear(&mut self) {
        self.elements = Vec::with_capacity(0);
        self.parents = Vec::with_capacity(0);
        self.len = 0;
    }

    fn union(&mut self, p: usize, q: usize) {
        todo!()
    }

    fn find_leader(&self, p: usize) -> usize {
        todo!()
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find_leader(p) == self.find_leader(q)
    }

    fn move_to(&mut self, p: usize, q: usize) {
        todo!()
    }

    fn count(&self) -> usize {
        self.len
    }
}
