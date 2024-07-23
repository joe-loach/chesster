#![feature(variant_count)]

mod bitboard;
mod fen;
mod iter;
mod piece;
mod square;

pub use bitboard::BitBoard;
pub use fen::FEN;
pub use piece::{Color, Piece, PieceKind};
pub use square::Square;

pub struct Board {
    pieces: [BitBoard; PieceKind::COUNT],
    colors: [BitBoard; Color::COUNT],
}

impl Board {
    /// Creates an empty [`Board`].
    pub fn empty() -> Self {
        Self {
            pieces: [BitBoard::EMPTY; PieceKind::COUNT],
            colors: [BitBoard::EMPTY; Color::COUNT],
        }
    }

    /// The starting board state for a game of chess.
    pub fn start() -> Self {
        const START_FEN: FEN =
            FEN::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        START_FEN
            .parse_board()
            .expect("Failed to parse FEN for starting position")
    }

    /// Returns an iterator over the boards [`Pieces`](Piece).
    pub fn iter(&self) -> iter::Iter {
        iter::Iter::new(self)
    }

    /// Finds the [`Piece`] on a [`Square`].
    ///
    /// If there is nothing on the [`Square`], None is returned.
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        let kind = self.kind_on(square)?;
        let is_white = self.whites().is_on(square);
        let color = if is_white { Color::White } else { Color::Black };

        Some(Piece::new().with_kind(kind).with_color(color))
    }

    /// Finds the kind of [`Piece`] on a square, without it's color.
    pub fn kind_on(&self, square: Square) -> Option<PieceKind> {
        PieceKind::ALL
            .into_iter()
            .find(|&p| self.pieces(p).is_on(square))
    }

    /// Finds the color of a [`Piece`] on a square.
    pub fn color_of(&self, square: Square) -> Option<Color> {
        Color::ALL
            .into_iter()
            .find(|&c| self.colors(c).is_on(square))
    }

    /// The [`BitBoard`] for a specific [`PieceKind`].
    #[inline]
    pub fn pieces(&self, kind: PieceKind) -> BitBoard {
        self.pieces[kind as usize]
    }

    /// A mutable [`BitBoard`] for a specific [`PieceKind`].
    #[inline]
    pub fn pieces_mut(&mut self, kind: PieceKind) -> &mut BitBoard {
        &mut self.pieces[kind as usize]
    }

    /// A [`BitBoard`] for all the pieces of a specific [`Color`].
    #[inline]
    pub fn colors(&self, color: Color) -> BitBoard {
        self.colors[color as usize]
    }

    /// A mutable [`BitBoard`] for all the pieces of a specific [`Color`].
    #[inline]
    pub fn colors_mut(&mut self, color: Color) -> &mut BitBoard {
        &mut self.colors[color as usize]
    }

    /// Toggles a [`Piece`] on a [`Square`].
    #[inline]
    pub fn toggle_square(&mut self, piece: Piece, square: Square) {
        self.pieces_mut(piece.kind()).toggle(square);
        self.colors_mut(piece.color()).toggle(square);
    }

    /// All occupied spaces are represented by this [`BitBoard`].
    ///
    /// This is the union of all black and white pieces.
    #[inline]
    pub fn occupied(&self) -> BitBoard {
        self.blacks() | self.whites()
    }

    /// A [`BitBoard`] of all **white** pieces positions.
    #[inline]
    pub fn whites(&self) -> BitBoard {
        self.colors(Color::White)
    }

    /// A [`BitBoard`] of all **black** pieces positions.
    #[inline]
    pub fn blacks(&self) -> BitBoard {
        self.colors(Color::Black)
    }

    /// All **pawn** positions.
    #[inline]
    pub fn pawns(&self) -> BitBoard {
        self.pieces(PieceKind::Pawn)
    }

    /// All *knight** positions.
    #[inline]
    pub fn knights(&self) -> BitBoard {
        self.pieces(PieceKind::Knight)
    }

    /// All **bishop** positions.
    #[inline]
    pub fn bishops(&self) -> BitBoard {
        self.pieces(PieceKind::Bishop)
    }

    /// All **rook** positions.
    #[inline]
    pub fn rooks(&self) -> BitBoard {
        self.pieces(PieceKind::Rook)
    }

    /// All **queen** positions.
    #[inline]
    pub fn queens(&self) -> BitBoard {
        self.pieces(PieceKind::Queen)
    }

    /// All **kings** positions.
    #[inline]
    pub fn kings(&self) -> BitBoard {
        self.pieces(PieceKind::King)
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, place) in self.iter().enumerate() {
            if let Some(piece) = place {
                write!(f, "{}", piece.as_char())?;
            } else {
                write!(f, " ")?;
            }
            if (i + 1) % 8 == 0 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
