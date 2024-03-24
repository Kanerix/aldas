use super::UnionFind;

/// The quick-find implementation have 2 vectores of equal size. 1 vector
/// for the elements, and 1 that stores each elements leader. Each element
/// in the set starts off by being its own leader. When connecting elements
/// together, they agree on a single "leader element". This is also called the
/// leader of an elements. All elements with the same leader, is considered a
/// disjoined set.
#[derive(Debug)]
pub struct QuickFind {
    leaders: Vec<usize>,
    len: usize,
}

impl UnionFind for QuickFind {
    fn new(n: usize) -> Self {
        QuickFind {
            leaders: vec![0; n],
            len: n,
        }
    }

    fn extend(&mut self, n: usize) {
        let count = self.count();
        let max = if count != 0 { count } else { 0 };

        self.leaders.extend(max..max + n);
        self.len += n - 1;
    }

    fn clear(&mut self) {
        self.leaders.clear();
        self.len = 0;
    }

    fn union(&mut self, p: usize, q: usize) {
        if let Some((p_leader, q_leader)) = self.find_leaders(p, q) {
            for e in 0..self.count() {
                let e_leader = self.leaders[e];

                if e_leader == p_leader {
                    self.leaders[e] = q_leader
                }
            }
        }
    }

    fn find_leader(&self, p: usize) -> usize {
        self.leaders[p]
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
            Some((p_leader, q_leader))
        }
    }

    fn move_to(&mut self, p: usize, q: usize) {
        if let Some((p_leader, q_leader)) = self.find_leaders(p, q) {
            let mut new_leader = p_leader;
            self.leaders[p] = q_leader;

            for e in 0..self.count() {
                if self.leaders[e] == p {
                    if new_leader == p {
                        new_leader = e
                    }
                    self.leaders[e] = new_leader;
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.len
    }
}
