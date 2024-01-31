//! # Theory
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
//! | [`QuickFind`]          | N            | a(N)   | a(N)  |

/// A trait containing methods to create an union find for a set.
///
/// A list of elements connected is called an component.
trait UnionFind {
    /// The element that is used by the set.
    ///
    /// Each entry in the set, is of this type.
    type Element;
    /// Returns a new set with elements from 0 up to `n` (`{0..n}`).
    ///
    /// No elements are connected, when the set is first created.
    /// Use the [`UnionFind::union`] method to combine two sets.
    ///
    /// The time complexity of this function is always `O(N)`.
    fn new(n: usize) -> Self;
    /// Connects two sets to each other.
    ///
    /// If `p` is a part of `q`'s set, this does nothing.
    fn union(&mut self, p: usize, q: usize);
    /// Finds the root element of `p`.
    ///
    /// Returns [`None`] if `p` is not a part of the set.
    fn find(&self, p: usize) -> Option<&Self::Element>;
    /// Checks if `p` is a part of `q`.
    ///
    /// Returns [`None`] if `p` or `q` is not a part of the entire set.
    /// Returns `true` if `q` and `p` is connected, otherwise `false`.
    fn connected(&self, p: usize, q: usize) -> Option<bool>;
    /// Returns a count of the amount of elements in the set.
    ///
    /// The time complexity of this function is always `O(n)`.
    fn count(&self) -> usize;
}

/// A simple element containing a value, and a pointer to its parent.
///
/// If the [`Element`] has no parent, it is [`None`].
/// This structure is used for performing a quick union.
struct Node {
    /// The value of the element in the set.
    value: usize,
    /// An optional pointer to the elements parent.
    ///
    /// A [`Box`] pointer to the parent provided in the [`Some(Box>Node>)`] variant.
    /// If the element does not have a parent, it is [`None`].
    parent: Option<Box<Node>>,
}

impl PartialEq for Node {
    /// Two elements are equal, if their `value` field is the same.
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Represents a quick find structure.
///
/// A naive implementation of a union find.
///
/// This only has good performance finding elements in the sets.
/// This implementation has poor performance performing a [`UnionFind::union`] on large sets.
struct QuickFind {
    elements: Vec<usize>,
    parents: Vec<usize>,
}

impl UnionFind for QuickFind {
    type Element = usize;

    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);
        let parents = Vec::with_capacity(n);

        for i in 0..elements.len() {
            elements.push(i);
        }

        QuickFind { elements, parents }
    }

    fn union(&mut self, p: usize, q: usize) {
        todo!()
    }

    fn find(&self, p: usize) -> Option<&Self::Element> {
        self.elements.get(p)
    }

    fn connected(&self, p: usize, q: usize) -> Option<bool> {
        todo!()
    }

    fn count(&self) -> usize {
        return self.elements.len();
    }
}

/// Represents a quick union structure.
struct QuickUnion {
    elements: Vec<Node>,
}

impl UnionFind for QuickUnion {
    type Element = Node;

    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);

        for i in 0..elements.len() {
            elements.push(Node {
                value: i,
                parent: None,
            });
        }

        QuickUnion { elements }
    }

    fn union(&mut self, p: usize, q: usize) {
        todo!()
    }

    fn find(&self, p: usize) -> Option<&Self::Element> {
        let mut root = &self.elements.get(p)?.parent;
        while let Some(element) = root {
            root = &element.parent;
        }
        return root.as_deref();
    }

    fn connected(&self, p: usize, q: usize) -> Option<bool> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}

/// Represents a weighted quick union structure.
///
/// This struct keeps track of the depth of each disjoined set.
/// When doing a union of two components, we make sure that the smaller
/// components is added to the bigger one.
struct WeightedQuickUnion {
    elements: Vec<Node>,
}

impl UnionFind for WeightedQuickUnion {
    type Element = Node;

    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);

        for i in 0..elements.len() {
            elements.push(Node {
                value: i,
                parent: None,
            });
        }

        WeightedQuickUnion { elements }
    }

    fn union(&mut self, p: usize, q: usize) {
        if self.connected(p, q).unwrap_or(false) {
            return;
        }
        todo!()
    }

    fn find(&self, p: usize) -> Option<&Self::Element> {
        let mut root = &self.elements.get(p)?.parent;
        while let Some(element) = root {
            root = &element.parent;
        }
        return root.as_deref();
    }

    fn connected(&self, p: usize, q: usize) -> Option<bool> {
        let p_root = self.find(p);
        let q_root = self.find(q);

        match (p_root, q_root) {
            (Some(p_root), Some(q_root)) => {
                return Some(p_root == q_root);
            }
            _ => return None,
        }
    }

    fn count(&self) -> usize {
        return self.elements.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = include_str!("data/disjoint_sets/1_in.txt");
    const OUTPUT_1: &str = include_str!("data/disjoint_sets/1_out.txt");

    #[test]
    fn test_quick_find() {
        let mut lines = INPUT_1.lines();

        let first_line = lines.next().unwrap().split(' ').collect::<Vec<&str>>();

        let n: usize = first_line[0].parse().unwrap();
        let m: usize = first_line[1].parse().unwrap();

        for line_raw in lines {
            let line = line_raw.split(' ').collect::<Vec<&str>>();
        }

        let output = "";

        assert_eq!(output, OUTPUT_1)
    }

    #[test]
    fn test_quick_union() {
        let output = "";

        todo!();

        assert_eq!(output, OUTPUT_1)
    }

    #[test]
    fn test_weighted_quick_union() {
        let output = "";

        todo!();

        assert_eq!(output, OUTPUT_1)
    }
}
