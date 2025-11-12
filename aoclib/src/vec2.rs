use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2({},{})", self.x, self.y)
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left
];

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

impl Direction {
    pub fn rot(self) -> Self {
        match self {
            Direction::Up => Direction::Right, // up -> right
            Direction::Right => Direction::Down, // right -> down
            Direction::Down => Direction::Left, // down -> left
            Direction::Left => Direction::Up, // left -> up
        }
    }

    pub fn inv(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right
        }
    }
}

impl Add<Vector2<usize>> for Direction {
    type Output = Vector2<usize>;

    fn add(self, rhs: Vector2<usize>) -> Self::Output {
        let mut r = rhs.clone();
        match self {
            Direction::Up => r.y -= 1,
            Direction::Right => r.x += 1,
            Direction::Down => r.y += 1,
            Direction::Left => r.x -= 1
        }

        r
    }
}

impl Add<Vector2<isize>> for Direction {
    type Output = Vector2<isize>;

    fn add(self, rhs: Vector2<isize>) -> Self::Output {
        let mut r = rhs.clone();
        match self {
            Direction::Up => r.y -= 1,
            Direction::Right => r.x += 1,
            Direction::Down => r.y += 1,
            Direction::Left => r.x -= 1
        }

        r
    }
}

#[test]
fn test_vec2_int() {
    let v1 = Vector2 { x: 1, y: 1 };
    let v2 = Vector2 { x: 2, y: 2 };
    let v3 = Vector2 { x: 3, y: 3 };
    let v4 = Vector2 { x: 4, y: 4 };

    assert_eq!(v1 + v2, v3);
    assert_eq!(v3 - v1, v2);
    assert_eq!(v2 * 2, v4);
}

#[test]
fn test_vec2_f64() {
    let v1 = Vector2 { x: 0.5, y: 0.5 };
    let v2 = Vector2 { x: 1.0, y: 1.0 };
    let v3 = Vector2 { x: 1.5, y: 1.5 };
    let v4 = Vector2 { x: 5.5, y: 5.5 };

    assert_eq!(v1 + v1, v2);
    assert_eq!((v2 * 4.0) + v3, v4);
    assert_eq!(v3 - v1, v2);
}