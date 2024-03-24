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
//! # Benchmarks
//!
//! Hyperfine benchmark results.
//!
//! ```
//! Benchmark 1: cargo test dset::tests::test_qf
//!  Time (mean ± σ):      4.615 s ±  0.229 s    [User: 4.534 s, System: 0.028 s]
//!  Range (min … max):    4.536 s …  5.267 s    10 runs
//! ```
//!
//! ```
//! Benchmark 2: cargo test dset::tests::test_qu
//!  Time (mean ± σ):     713.7 ms ±   2.0 ms    [User: 697.2 ms, System: 8.3 ms]
//!  Range (min … max):   711.6 ms … 718.1 ms    10 runs
//! ```
//!
//! ```
//! Benchmark 3: cargo test dset::tests::test_wqu
//!  Time (mean ± σ):     614.9 ms ±   1.6 ms    [User: 599.0 ms, System: 8.2 ms]
//!  Range (min … max):   613.2 ms … 618.5 ms    10 runs
//! ```
//!
//! ```
//! cargo test dset::tests::test_wqu ran
//!  1.16 ± 0.00 times faster than cargo test dset::tests::test_qu
//!  7.51 ± 0.37 times faster than cargo test dset::tests::test_qf
//! ```

pub mod qf;
pub mod qu;
pub mod wqu;

/// A trait containing methods to create an union find for a set.
///
/// A list of elements connected is called an component.
pub trait UnionFind {
    /// Returns a new set with elements from 0 up to `n` (`{0..n-1}`).
    ///
    /// No elements are connected, when the set is first created.
    /// This means each element is a disjoined set. Use the
    /// [`UnionFind::union`] method to combine two disjoined sets.
    ///
    /// The time complexity of this function is always `O(N)`.
    fn new(n: usize) -> Self;
    /// Extends the set with `n` new elements (`{0..len(self) + n - 1}`).
    fn extend(&mut self, n: usize);
    /// Removes all elements from the set.
    fn clear(&mut self);
    /// Connects two sets into one disjoined set. Also called a union of two sets.
    ///
    /// If `p` is a part of `Q` already (`p` ∈ `Q`), this does nothing.
    ///
    /// Math: `P` ⋃ `Q` = `S`
    fn union(&mut self, p: usize, q: usize);
    /// Returns the leader element of `p`.
    fn find_leader(&self, p: usize) -> usize;
    /// Checks if `p` is a part of `Q` (and wise versa).
    ///
    /// Returns `true` if `p` is a part of `Q` (`p` ∈ `Q`).
    fn connected(&self, p: usize, q: usize) -> bool;
    /// Get a pair of leaders that are not connected.
    ///
    /// Returns `None` if elements are connected.
    fn find_leaders(&self, p: usize, q: usize) -> Option<(usize, usize)>;
    /// Moves an element from `p`'s set to `q`'s set.
    fn move_to(&mut self, p: usize, q: usize);
    /// Returns a count of elements in the set.
    fn count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use self::{qf::QuickFind, qu::QuickUnion, wqu::WeightedQuickUnion};

    use super::*;

    const IN_1: &str = include_str!("samples/1.in");
    const IN_2: &str = include_str!("samples/2.in");
    const IN_3: &str = include_str!("samples/3.in");
    const IN_4: &str = include_str!("samples/4.in");
    const ANS_1: &str = include_str!("samples/1.ans");
    const ANS_2: &str = include_str!("samples/2.ans");
    const ANS_3: &str = include_str!("samples/3.ans");
    const ANS_4: &str = include_str!("samples/4.ans");

    #[test]
    fn test_qf() {
        let mut qf = QuickFind::new(0);
        union_find(&mut qf);
    }

    #[test]
    fn test_qu() {
        let mut qu = QuickUnion::new(0);
        union_find(&mut qu);
    }

    #[test]
    fn test_wqu() {
        let mut wqu = WeightedQuickUnion::new(0);
        union_find(&mut wqu);
    }

    fn union_find(union_find: &mut impl UnionFind) {
        for (input, anwser) in [(IN_1, ANS_1), (IN_2, ANS_2), (IN_3, ANS_3), (IN_4, ANS_4)] {
            let mut lines = input.lines();

            let first_line = lines.next().unwrap().split(' ').collect::<Vec<&str>>();
            let n: usize = first_line[0].parse().unwrap();
            let _: usize = first_line[1].parse().unwrap();

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

            assert_eq!(output.trim_end(), anwser.trim_end());
            union_find.clear();
        }
    }
}
