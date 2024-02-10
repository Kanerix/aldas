//! # Theory
//!
//! A disjoined set data structure, is a set where each element is its
//! own disjoined set. You can connect two set with the union operation.
//! This "connects" the two sets and make them 1 disjoined set.
//!
//! # Table
//!
//! | Algorithm              | Construction | Union  | Find  |
//! | ---------------------- | ------------ | ------ | ----- |
//! | [`QuickFind`]          | N            | N      | 1     |
//! | ---------------------- | ------------ | ------ | ----- |
//! | [`QuickUnion`]         | N            | depth  | depth |
//! | ---------------------- | ------------ | ------ | ----- |
//! | [`WeightedQuickUnion`] | N            | lg N   | lg N  |
//! | ---------------------- | ------------ | ------ | ----- |
//!

// pub mod ptr;
pub mod qfind;
pub mod qunion;
pub mod wqunion;

/// A trait containing methods to create an union find for a set.
///
/// A list of elements connected is called an component.
pub(crate) trait UnionFind {
    /// Returns a new set with elements from 0 up to `n` (`{0..n-1}`).
    ///
    /// No elements are connected, when the set is first created.
    /// This means each element is a disjoined set. Use the
    /// [`UnionFind::union`] method to combine two disjoined sets.
    ///
    /// The time complexity of this function is always `O(N)`.
    fn new(n: usize) -> Self;
    /// Extends the set with `n` elements.
    fn extend(&mut self, n: usize);
    /// Makes two sets into one. You can say this method connects two set.
    ///
    /// If `p` is a part of `q`'s set already, this does nothing.
    fn union(&mut self, p: usize, q: usize);
    /// Returns a pointer to the root element of `p`.
    fn find_leader(&self, p: usize) -> usize;
    /// Checks if `p` is a part of `q`.
    fn connected(&self, p: usize, q: usize) -> bool;
    /// Moves an element from `p`'s set to `q`'s set
    fn move_to(&mut self, p: usize, q: usize);
    /// Returns a count of the amount of elements in the set.
    fn count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use self::{qfind::QuickFind, qunion::QuickUnion, wqunion::WeightedQuickUnion};

    use super::*;

    const IN_1: &str = include_str!("samples/1.in");
    const ANS_1: &str = include_str!("samples/1.ans");

    const IN_2: &str = include_str!("samples/2.in");
    const ANS_2: &str = include_str!("samples/2.ans");

    const IN_3: &str = include_str!("samples/3.in");
    const ANS_3: &str = include_str!("samples/3.ans");

    const IN_4: &str = include_str!("samples/4.in");
    const ANS_4: &str = include_str!("samples/4.ans");

    #[test]
    fn test_quick_find() {
        let mut qf = QuickFind::new(0);
        union_find(&mut qf);
    }

    #[test]
    fn test_quick_union() {
        let mut qu = QuickUnion::new(0);
        union_find(&mut qu);
    }

    #[test]
    fn test_weighted_quick_union() {
        // TODO: Implement
        let mut wqu = WeightedQuickUnion::new(0);
        union_find(&mut wqu);
    }

    fn union_find(union_find: &mut impl UnionFind) {
        let mut lines = IN_4.lines();
        let first_line = lines.next().unwrap().split(' ').collect::<Vec<&str>>();
        let n: usize = first_line[0].parse().unwrap();
        let m: usize = first_line[1].parse().unwrap();

        union_find.extend(n);

        let mut output = String::new();
        for line_raw in lines {
            let line = line_raw.split(' ').collect::<Vec<&str>>();

            let operation = line[0];
            let s: usize = line[1].parse().unwrap();
            let t: usize = line[2].parse().unwrap();

            match operation {
                "0" => {
                    if union_find.connected(s, t) {
                        output.push_str("1\n");
                    } else {
                        output.push_str("0\n");
                    }
                }
                "1" => union_find.union(s, t),
                "2" => union_find.move_to(s, t),
                _ => unreachable!(),
            }
        }

        assert_eq!(output.trim_end(), ANS_4)
    }
}
