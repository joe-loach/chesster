use bitfield_struct::bitfield;

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct Piece {
    #[bits(1)]
    pub color: Color,
    #[bits(7)]
    pub kind: PieceKind,
}

impl Piece {
    /// Creates a new [`Piece`] with [`Color`] and [`PieceKind`].
    pub const fn new_with(color: Color, kind: PieceKind) -> Self {
        Self::new().with_color(color).with_kind(kind)
    }

    /// Returns `true` if the piece is white.
    pub const fn is_white(&self) -> bool {
        matches!(self.color(), Color::White)
    }

    /// Returns `true` if the piece is black
    pub const fn is_black(&self) -> bool {
        matches!(self.color(), Color::Black)
    }

    pub(crate) const fn as_char(&self) -> char {
        const PIECES: [char; 6] = ['p', 'n', 'b', 'r', 'q', 'k'];
        let c = PIECES[self.kind() as usize];
        if self.is_white() {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// White Pieces
    White = 0,
    /// Black Pieces
    Black = 1,
}

impl Color {
    pub(crate) const COUNT: usize = std::mem::variant_count::<Self>();

    pub(crate) const ALL: [Self; Self::COUNT] = [Self::White, Self::Black];

    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        let value = value as usize;

        if value < Self::COUNT {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("failed to create Color from bits, out of range");
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    /// All pawns
    Pawn = 0,
    /// All knights
    Knight = 1,
    /// All bishops
    Bishop = 2,
    /// All rooks
    Rook = 3,
    /// All queens
    Queen = 4,
    /// All kings
    King = 5,
}

impl PieceKind {
    pub(crate) const COUNT: usize = std::mem::variant_count::<Self>();

    pub(crate) const ALL: [Self; Self::COUNT] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];

    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        let value = value as usize;

        if value < Self::COUNT {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("failed to create PieceKind from bits, out of range");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_creation() {
        let p = Piece::new_with(Color::White, PieceKind::King);
        assert_eq!(p.color(), Color::White);
        assert_eq!(p.kind(), PieceKind::King);
    }
}

/// Convience macro for creating pieces in pseudo-FEN notation
/// 
/// # Example
/// ```rust
/// # use board::Color::*;
/// # use board::PieceKind::*;
/// # use board::piece;
/// let pawn = piece!(P);
/// assert_eq!(pawn.kind(), Pawn);
/// assert_eq!(pawn.color(), White);
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! piece {
    // White pieces
    (P) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::Pawn) };
    (N) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::Knight) };
    (B) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::Bishop) };
    (R) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::Rook) };
    (Q) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::Queen) };
    (K) => { $crate::Piece::new_with($crate::Color::White, $crate::PieceKind::King) };
    // Black pieces
    (p) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::Pawn) };
    (n) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::Knight) };
    (b) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::Bishop) };
    (r) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::Rook) };
    (q) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::Queen) };
    (k) => { $crate::Piece::new_with($crate::Color::Black, $crate::PieceKind::King) };
}

pub(crate) use piece;
