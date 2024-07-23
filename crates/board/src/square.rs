#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Create a [`Square`] from `rank` and `file`.
    /// 
    /// # Panics
    /// 
    /// Panics if either rank or file are out of range 1..=8
    #[inline]
    pub fn new(rank: u8, file: u8) -> Self {
        assert!(1 <= rank && rank <= 8, "rank should be between 1..=8");
        assert!(1 <= file && file <= 8, "file should be between 1..=8");

        let raw = (rank - 1) * 8 + (file - 1);

        Self::from_raw(raw)
    }

    /// Creates a [`Square`] from a raw value.
    /// 
    /// # Panics
    /// 
    /// Panics if the value cannot be represented as an enum variant.
    #[inline]
    pub fn from_raw(raw: u8) -> Self {
        assert!(
            raw < core::mem::variant_count::<Self>() as u8,
            "raw value must be a valid enum variant"
        );
        // SAFETY:
        // Checked that raw value can be casted to a valid variant
        // Both raw and square are the same repr type (u8)
        unsafe { core::mem::transmute(raw) }
    }

    /// Tries to create a [`Square`] from a raw value.
    /// 
    /// If it fails, None is returned.
    #[inline]
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        if raw < core::mem::variant_count::<Self>() as u8 {
            Some(Self::from_raw(raw))
        } else {
            None
        }
    }

    /// Gets the correct bit for the [`Square`].
    #[inline]
    pub fn bit(&self) -> u64 {
        1 << self.as_u8()
    }

    #[inline]
    fn as_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range_creation() {
        assert_eq!(Square::new(1, 1), Square::A1);
        assert_eq!(Square::new(8, 8), Square::H8);
    }

    #[test]
    #[should_panic]
    fn out_of_range_creation() {
        Square::new(0, 0);
        Square::new(9, 9);
    }

    #[test]
    fn correct_bits() {
        assert_eq!(Square::A1.bit(), 1);
        assert_eq!(Square::H8.bit(), 1 << 63);
    }
}
