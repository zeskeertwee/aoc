use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};
use bitvec::order::Lsb0;
use crate::vec2::{Vector2, DIRECTIONS};
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

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.grid[(self.width * row)..(self.width * (row + 1))]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(move |row| self.get_row(row))
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