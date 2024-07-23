use crate::{piece::piece, square::Square, Board};
use std::borrow::Cow;

#[derive(PartialEq, Eq, Clone)]
pub struct FEN<'a>(Cow<'a, [u8]>);

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum ParseError {
    #[error("encountered unknown characer")]
    UnknownChar,
    #[error("too little information in rank")]
    TooLittleRankInfo,
    #[error("too much information in rank")]
    TooMuchRankInfo,
}

impl<'a> FEN<'a> {
    pub const fn from_str(raw: &'a str) -> Self {
        assert!(raw.is_ascii(), "FEN strings must be ASCII");
        Self(Cow::Borrowed(raw.as_bytes()))
    }

    pub fn from_string(raw: String) -> Self {
        assert!(raw.is_ascii(), "FEN strings must be ASCII");
        Self(Cow::Owned(raw.into_bytes()))
    }

    pub fn parse_board(self) -> Result<Board, ParseError> {
        let mut it = self.0.into_iter();

        let mut board = Board::empty();

        let mut rank = 0_u8;
        let mut file = 0_u8;

        // parse board position
        'parsing: while let Some(&b) = it.next() {
            let raw = rank * 8 + file;
            if raw >= 64 {
                break 'parsing;
            }
            let square = Square::from_raw(raw);

            match b {
                b'p' => board.toggle_square(piece!(p), square),
                b'n' => board.toggle_square(piece!(n), square),
                b'b' => board.toggle_square(piece!(b), square),
                b'r' => board.toggle_square(piece!(r), square),
                b'q' => board.toggle_square(piece!(q), square),
                b'k' => board.toggle_square(piece!(k), square),
                b'P' => board.toggle_square(piece!(P), square),
                b'N' => board.toggle_square(piece!(N), square),
                b'B' => board.toggle_square(piece!(B), square),
                b'R' => board.toggle_square(piece!(R), square),
                b'Q' => board.toggle_square(piece!(Q), square),
                b'K' => board.toggle_square(piece!(K), square),
                b'/' => {
                    match file.cmp(&8) {
                        // next rank
                        std::cmp::Ordering::Equal => {
                            rank += 1;
                            file = 0;
                            continue 'parsing;
                        }
                        // too little info for rank
                        std::cmp::Ordering::Less => return Err(ParseError::TooLittleRankInfo),
                        // too much info for rank
                        std::cmp::Ordering::Greater => return Err(ParseError::TooMuchRankInfo),
                    }
                }
                n @ b'1'..=b'8' => {
                    let inc = n - b'0';
                    file += inc;
                    continue 'parsing;
                }
                _ => return Err(ParseError::UnknownChar),
            }

            file += 1;
        }

        // TODO: parse rest of game state

        Ok(board)
    }
}

impl Board {
    pub fn to_fen(&self) -> FEN {
        struct EmptyCounter {
            count: u32,
        }

        impl EmptyCounter {
            const NEW: Self = EmptyCounter { count: 0 };

            fn inc(&mut self) {
                self.count += 1;
            }

            fn push_if_needed(&mut self, fen: &mut String) {
                if self.count > 0 {
                    fen.push(char::from_digit(self.count, 10).unwrap());
                    self.count = 0;
                }
            }
        }

        let mut fen = String::new();
        let mut file = 0_u32;
        let mut rank = 0_u32;
        let mut empties = EmptyCounter::NEW;

        for piece in self.iter() {
            file += 1;

            if let Some(piece) = piece {
                // push any empty squares before new piece
                empties.push_if_needed(&mut fen);

                fen.push(piece.as_char());
            } else {
                // increment empties when there is no piece
                empties.inc();
            }

            if file == 8 {
                // at a new rank, push empty count before
                empties.push_if_needed(&mut fen);

                file = 0;
                rank += 1;

                // only push a '/' when there is another rank to come
                if rank < 8 {
                    fen.push('/');
                }
            }
        }

        FEN::from_string(fen)
    }
}

impl<'a> std::fmt::Debug for FEN<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FEN")
            .field(&std::str::from_utf8(&self.0).unwrap())
            .finish()
    }
}

#[test]
fn parse_round_trip() {
    // TODO: change this to include game state
    // let fen = FEN::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let fen = FEN::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    let board = fen
        .clone()
        .parse_board()
        .expect("FEN string was not parsed to board correctly");

    assert_eq!(fen, board.to_fen(), "FEN conversion should be lossless");
}
