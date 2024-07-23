use crate::{square::Square, Board, Piece};

pub struct Iter<'a> {
    pos: u8,
    board: &'a Board,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self { pos: 0, board }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Option<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(square) = Square::try_from_raw(self.pos) {
            let piece = self.board.piece_on(square);
            self.pos += 1;
            Some(piece)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remainder = (64 - self.pos) as usize;
        (remainder, Some(remainder))
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl<'a> IntoIterator for &'a Board {
    type Item = Option<Piece>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct IntoIter {
    pos: u8,
    board: Board,
}

impl IntoIter {
    pub(crate) fn new(board: Board) -> Self {
        Self { pos: 0, board }
    }
}

impl Iterator for IntoIter {
    type Item = Option<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(square) = Square::try_from_raw(self.pos) {
            let piece = self.board.piece_on(square);
            self.pos += 1;
            Some(piece)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remainder = (64 - self.pos) as usize;
        (remainder, Some(remainder))
    }
}

impl ExactSizeIterator for IntoIter {}

impl IntoIterator for Board {
    type Item = Option<Piece>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[test]
fn api() {
    let board = Board::start();
    let mut iter = board.into_iter();

    assert_eq!(iter.len(), 64);
    assert_eq!(
        iter.next(),
        Some(Some(
            Piece::new()
                .with_color(crate::Color::Black)
                .with_kind(crate::PieceKind::Rook)
        ))
    );
    assert_eq!(iter.len(), 63);
}
