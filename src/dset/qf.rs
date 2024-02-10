use super::UnionFind;

/// The quick-find implementation have 2 vectores of equal size. 1 vector
/// for the elements, and 1 that stores each elements leader. Each element
/// in the set starts off by being its own leader. When connecting elements
/// together, they agree on a single "leader element". This is also called the
/// leader of an elements. All elements with the same leader, is considered a
/// disjoined set.
#[derive(Debug)]
pub struct QuickFind {
    elements: Vec<usize>,
    leaders: Vec<usize>,
    len: usize,
}

impl UnionFind for QuickFind {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut leaders = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            leaders.push(i);
        }

        QuickFind {
            elements,
            leaders,
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
            self.leaders.push(i);
        }

        self.len += n - 1;
    }

    fn clear(&mut self) {
        self.elements = Vec::with_capacity(0);
        self.leaders = Vec::with_capacity(0);
        self.len = 0;
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_leader = self.leaders[p];
        let q_leader = self.leaders[q];

        for element in self.elements.iter() {
            let element_leader = self.leaders[*element];

            if element_leader == p_leader {
                self.leaders[*element] = q_leader
            }
        }
    }

    fn find_leader(&self, p: usize) -> usize {
        self.leaders[p]
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find_leader(p) == self.find_leader(q)
    }

    fn move_to(&mut self, p: usize, q: usize) {
        if self.connected(p, q) {
            return;
        }

        let mut new_leader = self.find_leader(p);
        self.leaders[p] = self.find_leader(q);

        for element in 0..self.count() {
            if self.leaders[element] == p {
                if new_leader == p {
                    new_leader = element
                }
                self.leaders[element] = new_leader;
            }
        }
    }

    fn count(&self) -> usize {
        self.len
    }
}
