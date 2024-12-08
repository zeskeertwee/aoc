use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
    }
}

impl Mul<i64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i64) -> Vector2 {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<i64> for Vector2 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
    }
}