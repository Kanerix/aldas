use super::UnionFind;

#[derive(Debug)]
pub struct QuickUnion {
    leaders: Vec<usize>,
    len: usize,
}

impl UnionFind for QuickUnion {
    fn new(n: usize) -> Self {
        QuickUnion {
            leaders: (0..n).collect(),
            len: n,
        }
    }

    fn extend(&mut self, n: usize) {
        let count = self.count();
        let max = if count != 0 { count - 1 } else { 0 };

        self.leaders.extend(max..max + n);

        self.len += n - 1;
    }

    fn clear(&mut self) {
        self.leaders = Vec::with_capacity(0);
        self.len = 0;
    }

    fn union(&mut self, p: usize, q: usize) {
        if let Some((p_leader, q_leader)) = self.find_leaders(p, q) {
            self.leaders[p_leader] = q_leader;
        }
    }

    fn find_leader(&self, p: usize) -> usize {
        let mut element = p;
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

    fn find_leaders(&self, p: usize, q: usize) -> Option<(usize, usize)> {
        let p_leader = self.find_leader(p);
        let q_leader = self.find_leader(q);

        if p_leader == q_leader {
            None
        } else {
            Some((p, q))
        }
    }

    fn move_to(&mut self, p: usize, q: usize) {
        if let Some((p_leader, q_leader)) = self.find_leaders(p, q) {
            let mut new_leader = p_leader;
            self.leaders[p] = q_leader;

            for element in 0..self.count() {
                if self.leaders[element] == p {
                    if new_leader == p {
                        new_leader = element
                    }
                    self.leaders[element] = new_leader;
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.len
    }
}
