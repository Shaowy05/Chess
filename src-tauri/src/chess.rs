use std::collections::HashMap;
use std::ops;

enum Direction {
    North, South, East, West
}

enum Colour {
    White, Black
}

enum PieceType {
    Pawn, Rook, Knight, Bishop, Queen, King
}

struct Piece {
    piece_type: PieceType,
    colour: Colour
}

#[derive(Debug)]
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

struct Position {
    rank: Rank,
    file: File
}

struct Board {
    pieces: HashMap<Position, Piece>,


}