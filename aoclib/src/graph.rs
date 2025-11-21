use std::cmp::{Eq, Ordering, Reverse};
use std::ops::AddAssign;
use std::hash::Hash;
use std::collections::BinaryHeap;
use rustc_hash::FxHashSet;

pub struct Graph<T, C> {
    pub edges: Vec<Edge<T, C>>,
    pub nodes: Vec<T>
}

pub struct Edge<T, C> {
    pub src: T,
    pub dst: T,
    pub cost: C
}

pub struct Path<T: Eq + Hash, C> {
    pub visited: FxHashSet<T>,
    pub position: T,
    pub cost: C
}

impl<T: Eq + Hash, C: PartialOrd> PartialOrd for Path<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<T: Eq + Hash, C: Ord> Ord for Path<T, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq + Hash, C: PartialEq> PartialEq for Path<T, C> {
    fn eq(&self, other: &Self) -> bool {
        other.cost.eq(&self.cost)
    }
}

impl<T: Eq + Hash, C: Eq> Eq for Path<T, C> {}

impl<T: Clone + Hash + Eq, C: Clone> Clone for Path<T, C> {
    fn clone(&self) -> Self {
        Self {
            visited: self.visited.clone(),
            position: self.position.clone(),
            cost: self.cost.clone()
        }
    }
}

impl<T: Clone + Hash + Eq, C: Copy> Graph<T, C> {
    // Create a new graph from a list of edges that are bidirectional
    // The edges will be inserted twice for each direction.
    pub fn new_bidirectional_edges(edges: Vec<Edge<T, C>>) -> Self {
        let edges = edges.into_iter()
            .map(|edge| [Edge {
                src: edge.dst.clone(),
                dst: edge.src.clone(),
                cost: edge.cost
            }, edge])
            .flatten()
            .collect();
        let nodes = get_all_nodes(&edges);

        Self {
            edges, nodes
        }
    }

    pub fn new(edges: Vec<Edge<T, C>>) -> Self {
        Self { nodes: get_all_nodes(&edges), edges }
    }
}

pub fn get_all_nodes<T: Clone + Hash + Eq, C: Clone>(edges: &Vec<Edge<T, C>>) -> Vec<T> {
    let mut nodes = FxHashSet::default();
    for i in edges {
        nodes.insert(i.src.clone());
        nodes.insert(i.dst.clone());
    }

    nodes.into_iter().collect()
}

impl<T: Eq + Hash + Clone, C: AddAssign<C> + PartialOrd + PartialEq + Ord + Default + Clone + Copy> Graph<T, C> {
    /// Finds the shortest path satisfying a condition
    pub fn bfs_find_shortest_path_length<F: Fn(&Path<T, C>, &Graph<T, C>) -> bool>(&self, cond: F) -> Option<C> {
        let mut queue: BinaryHeap<Path<T, C>> = BinaryHeap::new();

        for i in &self.nodes {
            queue.push(Path {
                visited: FxHashSet::default(),
                position: i.clone(),
                cost: C::default()
            });
        }

        while !queue.is_empty() {
            let path = queue.pop().unwrap();

            if cond(&path, &self) {
                return Some(path.cost);
            }

            for edge in &self.edges {
                if edge.src == path.position && !path.visited.contains(&edge.dst) {
                    // Move to the new position via the edge
                    let mut new_path = path.clone();
                    new_path.visited.insert(new_path.position);
                    new_path.position = edge.dst.clone();
                    new_path.cost += edge.cost;
                    queue.push(new_path);
                }
            }
        }

        None
    }

    /// finds the longest path visiting all nodes
    pub fn find_longest_path_visiting_all(&self) -> C {
        let mut queue: BinaryHeap<Reverse<Path<T, C>>> = BinaryHeap::new();
        let number_of_nodes = self.nodes.len();

        for i in &self.nodes {
            queue.push(Reverse(Path {
                visited: FxHashSet::default(),
                position: i.clone(),
                cost: C::default()
            }));
        }

        let mut most_expensive_path = C::default();

        while !queue.is_empty() {
            let path = queue.pop().unwrap();

            if path.0.visited.len() + 1 == number_of_nodes && path.0.cost > most_expensive_path {
                most_expensive_path = path.0.cost;
                continue;
            }

            for edge in &self.edges {
                if edge.src == path.0.position && !path.0.visited.contains(&edge.dst) {
                    // Move to the new position via the edge
                    let mut new_path = path.clone().0;
                    new_path.visited.insert(new_path.position);
                    new_path.position = edge.dst.clone();
                    new_path.cost += edge.cost;
                    queue.push(Reverse(new_path));
                }
            }
        }

        most_expensive_path
    }
}