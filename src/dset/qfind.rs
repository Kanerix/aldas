use super::UnionFind;

/// The quick-find implementation have 2 vectores of equal size. 1 vector
/// for the elements, and 1 that stores each elements parent. Each element
/// in the set starts off by being its own parent. When connecting elements
/// together, they agree on a single "leader element". This is also called the
/// parent of the elements. All elements with the same parent, is considered a
/// disjoined set.
pub struct QuickFind {
    elements: Vec<usize>,
    parents: Vec<usize>,
    len: usize,
}

impl UnionFind for QuickFind {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            parents.push(i);
        }

        QuickFind {
            elements,
            parents,
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

        self.elements.extend(max..max + n);
        self.parents.extend(max..max + n);
        self.len += n;
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_leader = self.parents[p];
        let q_leader = self.parents[q];

        for element in self.elements.iter() {
            let element_parent = self.parents[*element];

            if element_parent == p_leader {
                self.parents[*element] = q_leader
            }
        }
    }

    fn find_leader(&self, p: usize) -> usize {
        return self.parents[p];
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find_leader(p) == self.find_leader(q)
    }

    fn move_to(&mut self, p: usize, q: usize) {
        if self.connected(p, q) {
            return;
        }

        let mut new_parent = self.find_leader(p);
        self.parents[p] = self.find_leader(q);

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
        self.len
    }
}
