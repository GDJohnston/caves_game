use std::ops::AddAssign;

pub struct Point (pub i16, pub i16);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {0: self.0 + rhs.0, 1: self.1 + rhs.1};
    }
}
