use std::fmt::Debug;
use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};
use trait_set::trait_set;

trait_set! {
    pub trait DisjointSetKey = Copy + Hash + Eq + Debug;
}

#[derive(Default)]
pub struct DisjointSetForest<V: DisjointSetKey> {
    nodes: Vec<Node>,
    value2index: HashMap<V, usize>,
    roots: BTreeSet<usize>,
}

impl<V: DisjointSetKey> DisjointSetForest<V> {
    pub fn make_set(&mut self, value: V) {
        let parent = self.nodes.len();
        self.nodes.push(Node { parent, size: 1 });
        self.value2index.insert(value, parent);
        self.roots.insert(parent);
    }

    pub fn num_roots(&self) -> usize {
        self.roots.len()
    }

    fn index_of(&mut self, v: &V) -> usize {
        if !self.value2index.contains_key(v) {
            self.make_set(*v);
        }
        self.value2index.get(v).copied().unwrap()
    }

    pub fn union(&mut self, v1: &V, v2: &V) {
        let index1 = self.index_of(v1);
        let f1 = self.find(index1);
        let index2 = self.index_of(v2);
        let f2 = self.find(index2);
        if f1 != f2 {
            if self.nodes[f1].size > self.nodes[f2].size {
                self.nodes[f2].parent = f1;
                self.nodes[f1].size += self.nodes[f2].size;
                self.roots.remove(&f2);
            } else {
                self.nodes[f1].parent = f2;
                self.nodes[f2].size += self.nodes[f1].size;
                self.roots.remove(&f1);
            }
        }
    }

    pub fn set_size(&mut self, v: &V) -> usize {
        let root = self.find(*self.value2index.get(v).unwrap());
        self.nodes[root].size
    }

    pub fn all_sizes(&self) -> impl Iterator<Item = usize> {
        self.roots.iter().map(|root| self.nodes[*root].size)
    }

    fn find(&mut self, index: usize) -> usize {
        let mut root = index;
        while self.nodes[root].parent != root {
            root = self.nodes[root].parent;
        }
        let mut current = index;
        while current != root {
            let temp = current;
            current = self.nodes[temp].parent;
            self.nodes[temp].parent = root;
        }
        root
    }
}

struct Node {
    parent: usize,
    size: usize,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::disjoint_set_forest::DisjointSetForest;

    #[test]
    fn test() {
        let mut forest = DisjointSetForest::default();
        for i in 0..10 {
            forest.make_set(i);
        }

        for (n1, n2) in [(1, 2), (2, 3), (3, 4), (5, 6), (7, 8), (5, 7), (4, 9)] {
            forest.union(&n1, &n2);
        }

        for (n, size) in [
            (0, 1),
            (1, 5),
            (2, 5),
            (3, 5),
            (4, 5),
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 5),
        ] {
            assert_eq!(forest.set_size(&n), size);
        }

        let mut root_sizes = forest.all_sizes().collect_vec();
        root_sizes.sort();
        assert_eq!(vec![1, 4, 5], root_sizes);
    }
}
