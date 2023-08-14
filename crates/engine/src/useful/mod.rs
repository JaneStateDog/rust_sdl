use core::ops::{
    Add,
    Sub,
};

#[derive(Clone, Copy, Debug)]
pub struct Vector2(pub i32, pub i32);

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}