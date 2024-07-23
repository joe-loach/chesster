use std::ops::{BitAnd, BitOr};

use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BitBoard(pub(crate) u64);

impl BitBoard {
    pub const EMPTY: Self = Self(0);

    /// Returns `true` if the square is occupied.
    #[inline]
    pub fn is_on(&self, square: Square) -> bool {
        (self.0 & square.bit()) != 0
    }

    /// Toggles a specific square (XOR).
    #[inline]
    pub fn toggle(&mut self, square: Square) {
        self.0 ^= square.bit()
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}