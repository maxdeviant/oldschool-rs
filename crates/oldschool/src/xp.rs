use std::iter::Sum;
use std::ops::{Add, Div, Sub};

use crate::{Level, xp_table};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Xp(pub f32);

impl Xp {
    pub const ZERO: Xp = Xp(0.);

    pub fn from_level(level: &Level) -> Xp {
        *xp_table().get(&level).unwrap()
    }

    pub fn max(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl From<i32> for Xp {
    fn from(xp: i32) -> Self {
        Self(xp as f32)
    }
}

impl From<Level> for Xp {
    fn from(level: Level) -> Self {
        Self::from_level(&level)
    }
}

impl Add for Xp {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sum for Xp {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Xp(0.), Add::add)
    }
}

impl Sub for Xp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Div for Xp {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
