pub const BOARD_SIZE: usize = 8;

pub mod pieces {
    pub const NONE: i8 = 0;
    pub const PAWN: i8 = 1;
    pub const KNIGHT: i8 = 2;
    pub const BISHOP: i8 = 3;
    pub const ROOK: i8 = 4;
    pub const QUEEN: i8 = 5;
    pub const KING: i8 = 6;
}

pub mod sides {
    pub const WHITE: i8 = 1;
    pub const BLACK: i8 = -1;
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    board: [i8; 64],
    side_to_play: i8,
}

#[derive(Debug, Copy, Clone)]
pub struct Move(usize, usize, i8);

fn coordinates_from_index(index: usize) -> (usize, usize) {
    return (index % BOARD_SIZE, index / BOARD_SIZE)
}

fn color_of(piece: i8) -> i8 {
    if piece == 0 {
        0
    }
    else if piece < 0 {
        -1
    }
    else {
        1
    }
}

impl Position {
    fn starting_position() -> Position {
        Position {
            board: [
                 pieces::ROOK,  pieces::KNIGHT,  pieces::BISHOP,  pieces::QUEEN,  pieces::KING,  pieces::BISHOP,  pieces::KNIGHT,  pieces::ROOK,
                 pieces::PAWN,  pieces::PAWN,    pieces::PAWN,    pieces::PAWN,   pieces::PAWN,  pieces::PAWN,    pieces::PAWN,    pieces::PAWN,
                 pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,   pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,
                 pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,   pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,
                 pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,   pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,
                 pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,   pieces::NONE,  pieces::NONE,    pieces::NONE,    pieces::NONE,
                -pieces::PAWN, -pieces::PAWN,   -pieces::PAWN,   -pieces::PAWN,  -pieces::PAWN, -pieces::PAWN,   -pieces::PAWN,   -pieces::PAWN,
                -pieces::ROOK, -pieces::KNIGHT, -pieces::BISHOP, -pieces::QUEEN, -pieces::KING, -pieces::BISHOP, -pieces::KNIGHT, -pieces::ROOK,
            ],
            side_to_play: sides::WHITE,
        }
    }

    fn generate_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];

        for square_index in 0..64 {
            result.append(&mut match self.board[square_index] {
                pieces::NONE => vec![],
                pieces::PAWN => self.pawn_moves(square_index),
                // f => panic!("Unknown figure id {}", f),
                _ => vec![],
            })
        }

        result
    }

    fn pawn_moves(&self, square: usize) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];

        if self.board[square + 7] < pieces::NONE {
            if 48 <= square && square <= 55 {
                result.push(Move(square, square + 7, pieces::KNIGHT));
                result.push(Move(square, square + 7, pieces::BISHOP));
                result.push(Move(square, square + 7, pieces::ROOK));
                result.push(Move(square, square + 7, pieces::QUEEN));
            }
            else {
                result.push(Move(square, square + 7, 0));
            }
        }

        if self.board[square + 9] < pieces::NONE {
            if 48 <= square && square <= 55 {
                result.push(Move(square, square + 9, pieces::KNIGHT));
                result.push(Move(square, square + 9, pieces::BISHOP));
                result.push(Move(square, square + 9, pieces::ROOK));
                result.push(Move(square, square + 9, pieces::QUEEN));
            }
            else {
                result.push(Move(square, square + 9, 0));
            }
        }

        if self.board[square + 8] == pieces::NONE {
            if 48 <= square && square <= 55 {
                result.push(Move(square, square + 8, pieces::KNIGHT));
                result.push(Move(square, square + 8, pieces::BISHOP));
                result.push(Move(square, square + 8, pieces::ROOK));
                result.push(Move(square, square + 8, pieces::QUEEN));
            }
            else {
                result.push(Move(square, square + 8, 0));

                if 8 <= square && square <= 15 && self.board[square + 16] == pieces::NONE {
                    result.push(Move(square, square + 16, 0))
                }
            }
        }

        result
    }
}

impl Move {
    fn apply(&self, mut position: Position) -> Position {
        let piece = position.board[self.from];
        position.board[self.from] = pieces::NONE;

        let (_, y) = coordinates_from_index(self.from);

        if (piece == pieces::PAWN && y == 6) || (piece == -pieces::PAWN && y == 1) {
            position.board[self.to] = self.promotion;
        }
        else {
            position.board[self.to] = piece;

            if piece == pieces::KING && self.to - self.from == 2 {
                position.board[self.to + 1] = pieces::NONE;
                position.board[self.to - 1] = pieces::ROOK;
            }
            else if piece == -pieces::KING && self.to - self.from == 2 {
                position.board[self.to + 1] = pieces::NONE;
                position.board[self.to - 1] = pieces::ROOK;
            }
            else if piece == pieces::KING && self.from - self.to == 2 {
                position.board[self.to - 2] = pieces::NONE;
                position.board[self.to + 1] = pieces::ROOK;
            }
            else if piece == -pieces::KING && self.from - self.to == 2 {
                position.board[self.to - 2] = pieces::NONE;
                position.board[self.to + 1] = pieces::ROOK;
            }
        }

        position.side_to_play *= -1;

        position
    }
}

fn main() {
    println!(
        "{:?}",
        Move(12, 28, 0).apply(Position::starting_position())
    );
}
