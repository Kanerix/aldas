use super::UnionFind;

pub struct QuickUnion {
    elements: Vec<usize>,
    parents: Vec<usize>,
}

impl UnionFind for QuickUnion {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            parents.push(i);
        }

        QuickUnion { elements, parents }
    }

    fn extend(&mut self, n: usize) {
        let count = self.count();
        let max = if count != 0 {
            self.elements[count - 1]
        } else {
            0
        };

        self.elements.extend(max..max + n);
        self.parents.extend(max..max + n);
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find_leader(p);
        let q_root = self.find_leader(q);

        if self.connected(p_root, q_root) {
            return;
        }

        self.parents[p_root] = q_root;
    }

    fn find_leader(&self, p: usize) -> usize {
        let mut element = self.elements[p];
        let mut parent = self.parents[p];

        while element != parent {
            element = parent;
            parent = self.parents[element];
        }

        return element;
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find_leader(p) == self.find_leader(q)
    }

    fn move_to(&mut self, p: usize, q: usize) {
        if self.connected(p, q) {
            return;
        }

        let mut new_parent = self.find_leader(p);
        self.parents[p] = q;

        for element in 0..self.count() {
            if self.parents[element] == p {
                if new_parent == p {
                    new_parent = element
                }
                self.parents[element] = new_parent;
            }
        }
    }

    fn count(&self) -> usize {
        return self.elements.len();
    }
}
