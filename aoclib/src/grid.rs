use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};
use bitvec::order::Lsb0;
use crate::vec2::{Vector2, DIRECTIONS, Direction};
use bitvec::vec::BitVec;

#[derive(Clone)]
pub struct Grid<T> {
    pub grid: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T: Display> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} grid:\n", self.width, self.height)?;
        for row in self.rows() {
            for i in row {
                write!(f, "{}", i)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


impl<T> Grid<T> {
    pub fn parse<F: Fn(char) -> T>(input: &str, func: F) -> Grid<T> {
        let height = input.lines().count();
        let grid: Vec<T> = input.lines().map(|l| l.chars().map(&func)).flatten().collect();
        Grid { height, width: grid.len() / height, grid }
    }
    
    pub fn from_vec(v: Vec<T>, height: usize) -> Grid<T> {
        Grid { height, width: v.len() / height, grid: v }
    }
    
    pub fn is_inside<V: PartialOrd<usize>>(&self, v: &Vector2<V>) -> bool {
        v.x >= 0 && v.y >= 0 && v.x < self.width && v.y < self.height
    }

    /// Calculates the array index corresponding to the vector position in the grid
    pub fn calculate_index(&self, v: &Vector2<usize>) -> usize {
        v.x + v.y * self.width
    }

    /// Calculates the vector position in the grid from the index in the array
    pub fn calculate_position(&self, idx: usize) -> Vector2<usize> {
        let rem = idx % self.width;
        Vector2::new(rem, (idx - rem) / self.width)
    }
    
    pub fn index_unchecked(&self, v: &Vector2<usize>) -> &T {
        unsafe { self.grid.get_unchecked(self.calculate_index(v)) }
    }
    
    pub fn iter_squares(&self) -> impl Iterator<Item = (&T, Vector2<usize>)> {
        (0..self.grid.len()).map(|idx| {
            let v = self.calculate_position(idx);
            (unsafe { self.grid.get_unchecked(idx) }, v)
        })
    }


    /// returns a list of the neighbor squares directly above/below/left/right
    pub fn neighbour_squares(&self, v: &Vector2<usize>) -> Vec<Vector2<usize>> {
        let mut neighbours = Vec::with_capacity(4);

        for dir in DIRECTIONS {
            let pos = dir + *v;
            if self.is_inside(&pos) {
                neighbours.push(pos);
            }
        }

        neighbours
    }

    /// returns a list of the neighbor squares that are directly above/below/left/right and the ones diagonally above/below left/right
    pub fn adjacent_neighbour_squares(&self, v: &Vector2<usize>) -> Vec<Vector2<usize>> {
        let mut neighbours = self.neighbour_squares(v);
        let diag_dir = [Direction::Up + Direction::Left, Direction::Up + Direction::Right, Direction::Down + Direction::Left, Direction::Down + Direction::Right];

        for p in diag_dir {
            let pos = *v + p;
            if self.is_inside(&pos) {
                neighbours.push(pos);
            }
        }

        neighbours
    }

    // returns a list off all the neighbour squares that are radius steps away.
    pub fn neighbour_squares_radius(&self, v: &Vector2<usize>, radius: usize) -> Vec<Vector2<usize>> {
        let mut neighbours = Vec::new();

        for x in 0..=radius {
            let dy = radius - x;
            let v1 = Vector2::new(v.x - x, v.y - dy);
            if self.is_inside(&v1) {
                neighbours.push(v1);
            }

            if x != radius {
                let v2 = Vector2::new(v.x - x, v.y + dy);
                if self.is_inside(&v2) {
                    neighbours.push(v2);
                }
            }

            if x != 0 {
                let v3 = Vector2::new(v.x + x, v.y - dy);
                if self.is_inside(&v3) {
                    neighbours.push(v3);
                }

                if x != radius {
                    let v4 = Vector2::new(v.x + x, v.y + dy);
                    if self.is_inside(&v4) {
                        neighbours.push(v4);
                    }
                }
            }
        }

        neighbours
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.grid[(self.width * row)..(self.width * (row + 1))]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(move |row| self.get_row(row))
    }
}

// cost and priority are seperate as they might not be the same
#[derive(PartialEq, Eq, Debug)]
struct Node {
    position: Vector2<usize>,
    cost: usize,
    priority: usize,
}

// both PartialOrd and Ord implementations are reversed since we want the BinaryHeap to pop the *lowest* cost element
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<T: Eq> Grid<T> {
    pub fn flood_fill(&self) -> Vec<Vec<Vector2<usize>>> {
        let mut regions = Vec::new();

        // index into array as x + y * height
        let mut visited: BitVec<u8, Lsb0> = BitVec::repeat(false, self.width * self.height);

        for (c, v) in self.iter_squares() {
            let index = v.x + v.y * self.height;
            if visited[index] {
                continue;
            }

            let mut region = Vec::new();
            let mut stack = vec![v];
            while let Some(v) = stack.pop() {
                let idx = self.calculate_index(&v);
                if visited[idx] {
                    continue;
                }
                visited.set(idx, true);
                region.push(v);
                for n in self.neighbour_squares(&v) {
                    let idx = self.calculate_index(&n);
                    if self.grid[idx] == *c && !visited[idx] {
                        stack.push(n);
                    }
                }
            }

            regions.push(region);
        }

        regions
    }
    
    /// Searches breadth-first to find the shortest path from start to target, only walking over allowed_item, and returns the amount of steps in the path, if found.
    pub fn bfs_find_path(&self, start: Vector2<usize>, target: Vector2<usize>, allowed_item: &T) -> Option<usize> {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        queue.push(Node {
            position: start,
            priority: 0,
            cost: 0
        });

        let mut visited: BitVec<u8, Lsb0> = BitVec::repeat(false, self.grid.len());

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            if node.position == target {
                return Some(node.cost);
            }

            for neighbor in self.neighbour_squares(&node.position) {
                let idx = self.calculate_index(&neighbor);
                if &self.grid[idx] == allowed_item && !visited[idx]  {
                    visited.set(idx, true);
                    queue.push(Node {
                        position: neighbor,
                        priority: node.cost + 1,
                        cost: node.cost + 1,
                    })
                }
            }
        }

        None
    }

    /// Searches breadth-first for the cheapest path to each node, stopping when the target is reached
    pub fn bfs_find_node_costs(&self, start: Vector2<usize>, target: Vector2<usize>, allowed_item: &T) -> Option<Grid<usize>> {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        queue.push(Node {
            position: start,
            priority: 0,
            cost: 0
        });

        let mut visited = vec![usize::MAX; self.grid.len()];
        visited[self.calculate_index(&start)] = 0;

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            if node.position == target {
                return Some(Grid {
                    height: self.height,
                    width: self.width,
                    grid: visited,
                });
            }

            for neighbor in self.neighbour_squares(&node.position) {
                let idx = self.calculate_index(&neighbor);
                if &self.grid[idx] == allowed_item && visited[idx] == usize::MAX  {
                    visited[idx] = node.cost + 1;
                    queue.push(Node {
                        position: neighbor,
                        priority: node.cost + 1,
                        cost: node.cost + 1,
                    })
                }
            }
        }

        None
    }
    
    pub fn astar_find_path(&self, start: Vector2<usize>, target: Vector2<usize>, allowed_item: &T) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        let heuristic = |v: &Vector2<usize>| { v.x.abs_diff(target.x) + v.y.abs_diff(target.y) };
        
        queue.push(Node {
            priority: heuristic(&start),
            position: start,
            cost: 0,
        });
        
        let mut visited: BitVec<u8, Lsb0> = BitVec::repeat(false, self.grid.len());
        
        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            if node.position == target {
                return Some(node.cost);
            }
            
            for neighbor in self.neighbour_squares(&node.position) {
                let idx = self.calculate_index(&neighbor);
                if &self.grid[idx] == allowed_item && !visited[idx] {
                    visited.set(idx, true);
                    let new_cost = node.cost + 1;
                    queue.push(Node {
                        priority: new_cost + heuristic(&neighbor),
                        position: neighbor,
                        cost: new_cost
                    })
                }
            }
        }
        
        None
    }

    pub fn find_first_occurance(&self, v: &T) -> Option<Vector2<usize>> {
        self.iter_squares().find(|(val, _)| *val == v).map(|(_, v)| v)
    }
}

impl <T: Clone> Grid<T> {
    pub fn fill(v: T, width: usize, height: usize) -> Grid<T> {
        Self {
            grid: vec![v; width * height],
            width, height
        }
    }
}

impl<T: Copy + Default> Grid<T> {
    pub fn rotate(&self) -> Grid<T> {
        // the amount of rows in the new grid needs to be the width of the old grid
        //let mut result: Vec<Vec<T>> = vec![Vec::with_capacity(self.height); self.width];
        let mut result = Grid {
            height: self.width,
            width: self.height,
            grid: vec![T::default(); self.grid.len()]
        };

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                result[&Vector2::new(y, x)] = self[&Vector2::new(x, y)];
            }
        }

        //Grid::from_vec(result.into_iter().flatten().collect(), self.width)
        result
    }
}

impl<T> Index<&Vector2<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, v: &Vector2<usize>) -> &Self::Output {
        &self.grid[self.calculate_index(v)]
    }
}

impl<T> IndexMut<&Vector2<usize>> for Grid<T> {
    fn index_mut(&mut self, v: &Vector2<usize>) -> &mut Self::Output {
        let idx = self.calculate_index(v);
        &mut self.grid[idx]
    }
}

#[cfg(test)]
use crate::vec2::Direction;

#[test]
fn test_grid() {
    let input = "AAAA\nBBCD\nBBCC\nEEEC";
    let grid = Grid::parse(input, |c| c);
    assert_eq!(grid.height, 4);
    assert_eq!(grid.width, 4);
    
    assert_eq!(grid.get_row(0), ['A', 'A', 'A', 'A']);
    assert_eq!(grid.get_row(3), ['E', 'E', 'E', 'C']);

    assert_eq!(grid[&Vector2::new(0, 0)], 'A');
    assert_eq!(grid[&Vector2::new(2, 2)], 'C');
    assert_eq!(grid[&(Direction::Up + (Direction::Left + Vector2::new(1, 1)))], 'A');
    assert_eq!(grid[&(Direction::Down + (Direction::Right + Vector2::new(1, 1)))], 'C');

    assert_eq!(grid.calculate_position(5), Vector2::new(1, 1));

    let rot = grid.rotate();
    assert_eq!(rot.height, 4);
    assert_eq!(rot.width, 4);
    assert_eq!(rot.get_row(0), ['E', 'B', 'B', 'A']);
    assert_eq!(rot.get_row(3), ['C', 'C', 'D', 'A']);
}