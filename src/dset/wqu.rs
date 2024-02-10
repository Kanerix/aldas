use super::UnionFind;

#[derive(Debug)]
pub struct WeightedQuickUnion {
    elements: Vec<usize>,
    leaders: Vec<usize>,
    ranks: Vec<usize>,
    len: usize,
}

impl UnionFind for WeightedQuickUnion {
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let mut leaders = Vec::with_capacity(n);
        let mut ranks = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(i);
            leaders.push(i);
            ranks.push(0);
        }

        WeightedQuickUnion {
            elements,
            leaders,
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
            self.leaders.push(i);
            self.ranks.push(0);
        }

        self.len += n - 1;
    }

    fn clear(&mut self) {
        self.elements = Vec::with_capacity(0);
        self.leaders = Vec::with_capacity(0);
        self.ranks = Vec::with_capacity(0);
        self.len = 0;
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_leader = self.find_leader(p);
        let q_leader = self.find_leader(q);

        if self.connected(p_leader, q_leader) {
            return;
        }

        if self.ranks[q_leader] < self.ranks[p_leader] {
            self.leaders[p_leader] = q_leader;
            self.ranks[q_leader] += 1;
        } else {
            self.leaders[q_leader] = p_leader;
            self.ranks[p_leader] += 1;
        }
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

    // Wrong implementation. This works, but fucks up the rank.
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
