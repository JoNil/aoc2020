use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Pos(self.0 + other.0, self.1 + other.1);
    }
}
