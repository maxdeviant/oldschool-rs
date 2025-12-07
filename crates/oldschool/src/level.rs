use std::iter::Sum;
use std::ops::Add;

use crate::{Xp, xp_table};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, derive_more::Display)]
pub struct Level(pub i32);

impl Level {
    pub fn from_xp(xp: &Xp) -> Self {
        for (level, xp_for_level) in xp_table().iter().rev() {
            if xp_for_level <= &xp {
                return *level;
            }
        }

        Self::default()
    }

    pub fn is_99(&self) -> bool {
        self.0 == 99
    }
}

impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}

impl From<Xp> for Level {
    fn from(xp: Xp) -> Self {
        Self::from_xp(&xp)
    }
}

impl Add for Level {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sum for Level {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Level(0), Add::add)
    }
}
