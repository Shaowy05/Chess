use std::collections::HashMap;
use std::{ops, vec};

mod default_board;
use default_board::generate_default_board;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest
}

impl Direction {
    pub fn unit(&self) -> Vector {
        match self {
            Self::North => Vector { 
                x: 0,
                y: 1
            },
            Self::NorthEast => Vector { 
                x: 1,
                y: 1
            },
            Self::East => Vector { 
                x: 1,
                y: 0
            },
            Self::SouthEast => Vector { 
                x: 1,
                y: -1
            },
            Self::South => Vector { 
                x: 0,
                y: -1
            },
            Self::SouthWest => Vector { 
                x: -1,
                y: -1
            },
            Self::West => Vector { 
                x: -1,
                y: 0
            },
            Self::NorthWest => Vector { 
                x: -1,
                y: 1
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum JumpDirection {
    NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW
}

impl JumpDirection {
    pub fn as_vector(&self) -> Vector {
        match self {
            Self::NNE => Vector { x: 1, y: 2 },
            Self::NEE => Vector { x: 2, y: 1 },
            Self::SEE => Vector { x: 2, y: -1 },
            Self::SSE => Vector { x: 1, y: -2 },
            Self::SSW => Vector { x: -1, y: -2 },
            Self::SWW => Vector { x: -2, y: -1 },
            Self::NWW => Vector { x: -2, y: 1 },
            Self::NNW => Vector { x: -1, y: 2 },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    White, Black
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, PartialEq, Clone)]
enum MoveType {
    Strafe(Direction, usize),
    Jump(JumpDirection)
}

impl MoveType {
    pub fn as_vector(&self) -> Vector {
        use MoveType::*;
        use Direction::*;
        use JumpDirection::*;

        return match self {
            Strafe(d, n) => d.unit() * *n as isize,
            Jump(d) => d.as_vector()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Piece {
    colour: Colour,
    piece_type: PieceType
}

struct Vector {
    x: isize,
    y: isize
}

impl ops::Add<Vector> for Vector {
    type Output = Vector; 

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector; 

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    rank: Rank,
    file: File
}

impl ops::Add<Vector> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Vector) -> Self::Output {
        let new_file: Option<File>;

        if rhs.x.is_negative() {
            new_file = self.file - rhs.x.unsigned_abs()
        } else {
            new_file = self.file + rhs.x.unsigned_abs()
        }
    
        let new_rank: Option<Rank>;

        if rhs.y.is_negative() {
            new_rank = self.rank - rhs.y.unsigned_abs()
        } else {
            new_rank = self.rank + rhs.y.unsigned_abs()
        }

        if new_file == None || new_rank == None {
            return None
        } else {
            return Some(Self {
                rank: new_rank.unwrap(),
                file: new_file.unwrap()
            })
        }

    }
}

#[derive(Debug, PartialEq, Clone)]
struct Move {
    from: Position,
    move_type: MoveType, 
    to: Option<Position>
}

impl Move {
    pub fn new(from: Position, move_type: MoveType) -> Self {

        let to = from + move_type.as_vector();

        Self {
            from,
            move_type,
            to  
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    pieces: HashMap<Position, Piece>,
}

impl Board {
    pub fn make_move(&self, m: Move) -> Option<Self> {
        if m.to == None { return None };

        let new_pos_to_piece: (Position, Piece);
        let mut new_pieces: HashMap<Position, Piece> = HashMap::default();
        let mut piece_to_move: Piece;
        let mut position_to_move_to: Position;

        for (position, piece) in self.pieces.clone() {

            if position == m.from {
                piece_to_move = piece;
                continue;
            }

            if position == m.to.unwrap() {
                position_to_move_to = position;
                continue;
            }

            new_pieces.insert(position, piece);
        }

        return Some(Board {
            pieces: new_pieces
        });
    }
}

#[derive(Debug, Clone)]
struct Turn {
    colour: Colour,
    move_made: Move,
    resultant_board: Board
}

struct Game {
    initial_board: Board,
    turns: Vec<Turn>
}

impl Game {
    pub fn play() -> Self {
        let mut turn_count: usize = 0;
        let mut turns: Vec<Turn> = vec![];

        loop {
            let colour: Colour;
            
            if turn_count % 2 == 0 {
                colour = Colour::White;
            } else {
                colour = Colour::Black;
            }

            let current_board = match turns.pop() {
                Some(b) => b.resultant_board,
                None => generate_default_board()
            };

            let pieces: Vec<Piece> = current_board.pieces
                .into_iter()
                .filter(|(_, p)| p.colour == colour)
                .map(|(_, p)| p)
                .collect();



        }
    }
}