use std::ops::{Add, AddAssign, Mul};

#[derive(Copy, Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vec2(pub i32, pub i32);

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Vec2(self.0 + other.0, self.1 + other.1);
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self * rhs.0, self * rhs.1)
    }
}
