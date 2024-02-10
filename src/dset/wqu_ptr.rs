use std::ptr::NonNull;

/// A disjoined-set data structure.
#[derive(Debug)]
struct DSetPtr {
    nodes: Vec<Box<Node>>,
    len: usize,
}

/// A node in the disjoined-set.
///
/// The element field holds the value of the element in the set.
/// If the parentfield is [`None`], this means that this element
/// is the root of that individual disjoined-set.
#[derive(Debug)]
struct Node {
    element: usize,
    parent: Option<NonNull<Node>>,
    rank: usize,
}

impl Node {
    /// Construct a new node.
    ///
    /// The node is boxed because we need to make sure,
    /// that the node is never moved to another place in memory.
    fn new(element: usize) -> Box<Self> {
        Box::new(Node {
            element,
            parent: None,
            rank: 0,
        })
    }

    /// Returns the element held by the [`Node`].
    fn into_element(&self) -> usize {
        self.element
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl DSetPtr {
    /// Construct a new disjoined set with elements from {0..`n`-1}.
    ///
    /// This uses [`Node::new`] to add elements, and therefor
    /// no sets are connected at first. You can use [`DisjoinedSet::union`]
    /// to create a union of 2 sets, connecting them to each other.
    ///
    /// The time complexity of the function will always be `O(n)`
    /// because we have to loop trough all elements and construct
    /// a new node to add to the disjoined set.
    fn new(n: usize) -> Self {
        let mut elements = Vec::with_capacity(n);

        for i in 0..n {
            elements.push(Node::new(i));
        }

        DSetPtr {
            nodes: elements,
            len: n,
        }
    }

    /// Extend the set with `n` new elements.
    ///
    /// This uses the last element in the disjoined set as a
    /// reference to the highest value in the disjoined set.
    ///
    /// This means that the last element in the entire disjoined set
    /// always has to be the highest element, otherwise this method
    /// does not work.
    ///
    /// The time complexity of this function is also `O(n)` because
    /// it has to loop through `n` elements and construct a new [`Node`]
    /// and add to the list of elements.
    fn extend(&mut self, n: usize) {
        let last_element = if self.len != 0 {
            self.nodes[self.len - 1].element
        } else {
            0
        };

        for i in last_element..last_element + n {
            self.nodes.push(Node::new(i));
        }

        self.len += n;
    }

    /// Gets the leader node of `p`'s disjoined set.
    ///
    /// The leader node will always be the only node that has no parent
    /// (the parent is [`None`]).
    ///
    /// This has the time complexity O(depth), where depth is how deep the
    /// [`Node`] is in the individual disjoined set tree, because we have to go
    /// through the parent of all elements until we hit an [`Node`] without a parent
    /// (this is the root node).
    fn get_leader(&self, p: usize) -> Option<Box<Node>> {
        let mut node = *self.nodes.get(p)?;
        while let Some(parent) = node.parent {
            node = unsafe { Box::from_raw(parent.as_ptr()) };
        }
        Some(node)
    }

    /// Returns if two elements is in the same individual set.
    fn connected(&self, p: usize, q: usize) -> Option<bool> {
        Some(self.get_leader(p)?.element == self.get_leader(q)?.element)
    }

    /// Creates a union of two sets.
    ///
    /// This returns [`None`] if `p` or `q` is a part if the entire set.
    /// In the [`Some`] variant, the boolean is `true` if the union was performed
    /// otherwise `false` is provided. An union will not be performed if
    /// `p` and `q` is in the same individual set.
    fn union(&mut self, p: usize, q: usize) -> Option<bool> {
        if self.connected(p, q)? {
            return Some(false);
        }

        let mut p_root_box = self.get_leader(p)?;
        let q_root_box = self.get_leader(q)?;

        let p_root: *mut _ = p_root_box.as_mut();
        let q_root = q_root_box.as_ref();

        unsafe { (*p_root).parent = Some(NonNull::from(q_root)) };

        Some(true)
    }

    /*fn move_to(&mut self, p: usize, q: usize) -> Option<bool> {
        if self.connected(p, q)? {
            return Some(false);
        }

        let mut parent_for_children = self.get_root(p)?;

        let q_root = self.get_root(q)?.as_mut();
        let p_node = self.nodes.get(p)?.as_mut();

        let p_node_ptr = unsafe { NonNull::new_unchecked(q_root_raw) };
        let p_node: &Node = unsafe { p_node_ptr.as_ref() };

        for node in self.nodes.iter_mut() {
            match node.parent {
                None => continue,
                Some(parent) => unsafe {
                    let parent: &Node = unsafe { parent.as_ref() };
                    if parent == p_node {
                        if parent_for_children == p_node {}
                    }
                },
            }
        }

        Some(true)
    }*/

    fn count(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("samples/1.in");
    const OUTPUT: &str = include_str!("samples/1.ans");

    #[test]
    fn test_union_find() {
        let mut lines = INPUT.lines();

        let first_line = lines.next().unwrap().split(' ').collect::<Vec<&str>>();

        let n: usize = first_line[0].parse().unwrap();
        let m: usize = first_line[1].parse().unwrap();

        let mut d_set = DSetPtr::new(n);

        let mut output = String::new();

        for line_raw in lines {
            let line = line_raw.split(' ').collect::<Vec<&str>>();

            let operation = line[0];

            let s: usize = line[1].parse().unwrap();
            let t: usize = line[2].parse().unwrap();

            match operation {
                "0" => {
                    if d_set.connected(s, t).unwrap() {
                        output.push_str("1\n");
                    } else {
                        output.push_str("0\n");
                    }
                }
                "1" => {
                    d_set.union(s, t);
                }
                _ => unreachable!(),
            }
        }

        assert_eq!(output.trim_end(), OUTPUT)
    }
}
