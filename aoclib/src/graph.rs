use std::cmp::{Eq, Ordering};
use std::ops::{AddAssign, Sub, Mul};
use std::hash::Hash;
use std::collections::BinaryHeap;
use rustc_hash::{FxHashMap, FxHashSet};

// TODO: use a matrix for costs instead of hashmap

pub struct Graph<T, C> {
    // src, edge
    pub edges: FxHashMap<T, Vec<Edge<T, C>>>,
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

impl<T: Clone, C: Clone> Clone for Edge<T, C> {
    fn clone(&self) -> Self {
        Self {
            src: self.src.clone(),
            dst: self.dst.clone(),
            cost: self.cost.clone()
        }
    }
}

impl<T: Clone, C: Clone> Clone for Graph<T, C> {
    fn clone(&self) -> Self {
        Self {
            edges: self.edges.clone(),
            nodes: self.nodes.clone()
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
            .fold(FxHashMap::default(), |mut acc: FxHashMap<T, Vec<Edge<T, C>>>, edge| {
                if let Some(list) = acc.get_mut(&edge.src) {
                    list.push(edge);
                } else {
                    acc.insert(edge.src.clone(), vec![edge]);
                }
                acc
            });
        let nodes = get_all_nodes(&edges);

        Self {
            edges, nodes
        }
    }

    pub fn new(edges: Vec<Edge<T, C>>) -> Self {
        let edges = edges.into_iter().fold(FxHashMap::default(), |mut acc: FxHashMap<T, Vec<Edge<T, C>>>, edge| {
            if let Some(list) = acc.get_mut(&edge.src) {
                list.push(edge);
            } else {
                acc.insert(edge.src.clone(), vec![edge]);
            }
            acc
        });
        Self { nodes: get_all_nodes(&edges), edges }
    }
}

pub fn get_all_nodes<T: Clone + Hash + Eq, C: Clone>(edges: &FxHashMap<T, Vec<Edge<T, C>>>) -> Vec<T> {
    edges.keys().map(|k| k.clone()).collect()
}

impl<T: Eq + Hash + Clone, C: AddAssign<C> + PartialOrd + PartialEq + Ord + Default + Clone + Copy + Sub<Output = C> + TryFrom<usize> + Mul<Output = C>> Graph<T, C> {
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

            for edge in self.edges.get(&path.position).unwrap() {
                if !path.visited.contains(&edge.dst) {
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

    /// finds the longest path satisfying a condition
    pub fn bfs_find_longest_path_length<F: Fn(&Path<T, C>, &Graph<T, C>) -> bool>(&self, cond: F) -> C {
        let mut graph = self.clone();
        let max_edge_cost = graph.edges.iter().map(|(_, edge)| edge).flatten().map(|v| v.cost).max().unwrap();

        for (_, edges) in graph.edges.iter_mut() {
            for edge in edges.iter_mut() {
                edge.cost = max_edge_cost - edge.cost;
            }
        }

        let cost = graph.bfs_find_shortest_path_length(cond).unwrap();
        (max_edge_cost * if let Ok(v) = C::try_from(graph.nodes.len() - 1) {
            v
        } else { panic!() }) - cost
    }
}