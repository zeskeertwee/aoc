use std::ops::{Index, IndexMut};
use crate::vec2::{Vector2, DIRECTIONS};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn parse<F: Fn(char) -> T>(input: &str, func: F) -> Grid<T> {
        let grid: Vec<Vec<T>> = input.lines().map(|l| l.chars().map(&func).collect()).collect();
        Grid { height: grid.len(), width: grid[0].len(), grid }
    }
    
    pub fn from_vec(v: Vec<Vec<T>>) -> Grid<T> {
        Grid { height: v.len(), width: v[0].len(), grid: v }
    }
    
    pub fn is_inside<V: PartialOrd<usize>>(&self, v: &Vector2<V>) -> bool {
        v.x >= 0 && v.y >= 0 && v.x < self.width && v.y < self.height
    }
    
    pub fn index_unchecked(&self, v: &Vector2<usize>) -> &T {
        unsafe { self.grid.get_unchecked(v.y).get_unchecked(v.x) }
    }
    
    pub fn iter_squares(&self) -> impl Iterator<Item = (&T, Vector2<usize>)> {
        (0..self.height).cartesian_product(0..self.width).map(|(x, y)| {
            let v = Vector2::new(x, y);
            (self.index_unchecked(&v), v)
        })
    }

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
}

impl<T: Copy> Grid<T> {
    pub fn rotate(&self) -> Grid<T> {
        // the amount of rows in the new grid needs to be the width of the old grid
        let mut result: Vec<Vec<T>> = vec![Vec::with_capacity(self.height); self.width];

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                result[x].push(self.grid[y][x]);
            }
        }

        Grid::from_vec(result)
    }
}

impl<T> Index<&Vector2<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Vector2<usize>) -> &Self::Output {
        &self.grid[index.y][index.x]
    }
}

impl<T> IndexMut<&Vector2<usize>> for Grid<T> {
    fn index_mut(&mut self, index: &Vector2<usize>) -> &mut Self::Output {
        &mut self.grid[index.y][index.x]
    }
}