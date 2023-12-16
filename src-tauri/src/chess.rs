use std::collections::HashMap;
use std::{ops, vec};

pub const DEFAULT_INITIAL_BOARD: Board = Board {
    pieces: vec![
        (
            Position {
                rank: Rank::I,
                file: File::A
            },
            Piece::Rook(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::B
            },
            Piece::Knight(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::C
            },
            Piece::Bishop(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::D
            },
            Piece::Queen(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::E
            },
            Piece::King(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::F
            },
            Piece::Bishop(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::G
            },
            Piece::Knight(Colour::White)
        ),
        (
            Position {
                rank: Rank::I,
                file: File::G
            },
            Piece::Rook(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::A
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::B
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::C
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::D
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::E
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::F
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::G
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::II,
                file: File::H
            },
            Piece::Pawn(Colour::White)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::A
            },
            Piece::Rook(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::B
            },
            Piece::Knight(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::C
            },
            Piece::Bishop(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::D
            },
            Piece::Queen(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::E
            },
            Piece::King(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::F
            },
            Piece::Bishop(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::G
            },
            Piece::Knight(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VIII,
                file: File::G
            },
            Piece::Rook(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::A
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::B
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::C
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::D
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::E
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::F
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::G
            },
            Piece::Pawn(Colour::Black)
        ),
        (
            Position {
                rank: Rank::VII,
                file: File::H
            },
            Piece::Pawn(Colour::Black)
        ),
    ].into_iter().collect()
};

enum Colour {
    White, Black
}

enum Piece {
    Pawn(Colour),
    Rook(Colour),
    Knight(Colour),
    Bishop(Colour),
    Queen(Colour),
    King(Colour)
}

struct Translation {
    north: usize,
    east: usize,
    south: usize,
    west: usize
}

impl Translation {
    // pub fn from_tuple((n, e, s, w): (usize, usize, usize, usize)) -> Self {
    //     Self { north: n, east: e, south: s, west: w }
    // }

    pub fn new(n: usize, e: usize, s: usize, w: usize) -> Self {
        Self { north: n, east: e, south: s, west: w }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    I, II, III, IV, V, VI, VII, VIII
}

impl ops::Add<usize> for Rank {
    type Output = Option<Rank>;

    fn add(self, rhs: usize) -> Self::Output {

        if rhs == 0 {
            return Some(self);
        }

        return match self {
            Rank::I => Rank::II.add(rhs - 1),
            Rank::II => Rank::III.add(rhs - 1),
            Rank::III => Rank::IV.add(rhs - 1),
            Rank::IV => Rank::V.add(rhs - 1),
            Rank::V => Rank::VI.add(rhs - 1),
            Rank::VI => Rank::VII.add(rhs - 1),
            Rank::VII => Rank::VIII.add(rhs - 1),
            Rank::VIII => None,
        }
    }    
}

impl ops::Sub<usize> for Rank {
    type Output = Option<Rank>;
    
    fn sub(self, rhs: usize) -> Self::Output {

        if rhs == 0 {
            return Some(self);
        }

        return match self {
            Rank::VIII => Rank::VII.sub(rhs - 1),
            Rank::VII => Rank::VI.sub(rhs - 1),
            Rank::VI => Rank::V.sub(rhs - 1),
            Rank::V => Rank::IV.sub(rhs - 1),
            Rank::IV => Rank::III.sub(rhs - 1),
            Rank::III => Rank::II.sub(rhs - 1),
            Rank::II => Rank::I.sub(rhs - 1),
            Rank::I => None,
        }
    }    
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum File {
    A, B, C, D, E, F, G, H
}

impl ops::Add<usize> for File {
    type Output = Option<File>;

    fn add(self, rhs: usize) -> Self::Output {
        
        if rhs == 0 {
            return Some(self);
        }

        return match self {
            File::A => File::B.add(rhs - 1),
            File::B => File::C.add(rhs - 1),
            File::C => File::D.add(rhs - 1),
            File::D => File::E.add(rhs - 1),
            File::E => File::F.add(rhs - 1),
            File::F => File::G.add(rhs - 1),
            File::G => File::H.add(rhs - 1),
            File::H => None,
        };

    }
}

impl ops::Sub<usize> for File {
    type Output = Option<File>;

    fn sub(self, rhs: usize) -> Self::Output {
        
        if rhs == 0 {
            return Some(self);
        }

        return match self {
            File::H => File::G.sub(rhs - 1),
            File::G => File::F.sub(rhs - 1),
            File::F => File::E.sub(rhs - 1),
            File::E => File::D.sub(rhs - 1),
            File::D => File::C.sub(rhs - 1),
            File::C => File::B.sub(rhs - 1),
            File::B => File::A.sub(rhs - 1),
            File::A => None,
        };

    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    rank: Rank,
    file: File
}

#[derive(PartialEq, Eq)]
struct Move {
    from: Position,
    to: Position
}

struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    fn make_move(&self, m: Move) -> Self {
        Self {
            pieces: self.pieces.into_iter().map(|x| {
                if x.0 == m.from {
                    (m.to, x.1)
                } else {
                    x
                }
            } ).into_iter().collect()
        }
    }
}

struct Game {
    initial_board: Board,
    turns
}