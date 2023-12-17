use super::*;

pub fn generate_default_board() -> Board {
    use Rank::*;
    use File::*;
    use Colour::*;
    use PieceType::*;

    Board {
        pieces: HashMap::from([
            (
                Position {
                    rank: I,
                    file: A
                },
                Piece {
                    colour: White,
                    piece_type: Rook 
                }
            ),
            (
                Position {
                    rank: I,
                    file: B
                },
                Piece {
                    colour: White,
                    piece_type: Knight
                }
            ),
            (
                Position {
                    rank: I,
                    file: C
                },
                Piece {
                    colour: White,
                    piece_type: Bishop
                }
            ),
            (
                Position {
                    rank: I,
                    file: D
                },
                Piece {
                    colour: White,
                    piece_type: Queen
                }
            ),
            (
                Position {
                    rank: I,
                    file: E
                },
                Piece {
                    colour: White,
                    piece_type: King
                }
            ),
            (
                Position {
                    rank: I,
                    file: F
                },
                Piece {
                    colour: White,
                    piece_type: Bishop
                }
            ),
            (
                Position {
                    rank: I,
                    file: G
                },
                Piece {
                    colour: White,
                    piece_type: Knight
                }
            ),
            (
                Position {
                    rank: I,
                    file: H
                },
                Piece {
                    colour: White,
                    piece_type: Rook
                }
            ),
            (
                Position {
                    rank: II,
                    file: A
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: B
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: C
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: D
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: E
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: F
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: G
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: II,
                    file: H
                },
                Piece {
                    colour: White,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: A
                },
                Piece {
                    colour: Black,
                    piece_type: Rook 
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: B
                },
                Piece {
                    colour: Black,
                    piece_type: Knight
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: C
                },
                Piece {
                    colour: Black,
                    piece_type: Bishop
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: D
                },
                Piece {
                    colour: Black,
                    piece_type: Queen
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: E
                },
                Piece {
                    colour: Black,
                    piece_type: King
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: F
                },
                Piece {
                    colour: Black,
                    piece_type: Bishop
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: G
                },
                Piece {
                    colour: Black,
                    piece_type: Knight
                }
            ),
            (
                Position {
                    rank: VIII,
                    file: H
                },
                Piece {
                    colour: Black,
                    piece_type: Rook
                }
            ),
            (
                Position {
                    rank: VII,
                    file: A
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: B
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: C
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: D
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: E
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: F
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: G
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
            (
                Position {
                    rank: VII,
                    file: H
                },
                Piece {
                    colour: Black,
                    piece_type: Pawn
                }
            ),
        ])
    }
}