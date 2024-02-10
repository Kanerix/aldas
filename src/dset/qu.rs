use super::UnionFind;

#[derive(Debug)]
pub struct QuickUnion {
    elements: Vec<usize>,
    leaders: Vec<usize>,
    len: usize,
}

impl UnionFind for QuickUnion {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut leaders = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            leaders.push(i);
        }

        QuickUnion {
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
        let p_leader = self.find_leader(p);
        let q_leader = self.find_leader(q);

        if self.connected(p_leader, q_leader) {
            return;
        }

        self.leaders[p_leader] = q_leader;
    }

    fn find_leader(&self, p: usize) -> usize {
        let mut element = self.elements[p];
        let mut leader = self.leaders[p];

        while element != leader {
            element = leader;
            leader = self.leaders[element];
        }

        element
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
