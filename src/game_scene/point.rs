use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point (pub i16, pub i16);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {0: self.0 + rhs.0, 1: self.1 + rhs.1};
    }
}

impl Point {
    pub fn clamp (mut self: Self, min: Self, max: Self) -> Self {
        self.0 = self.0.clamp(min.0, max.0);
        self.1 = self.1.clamp(min.1, max.1);
        self
    }
}