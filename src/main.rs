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

pub struct Move {
    from: usize,
    to: usize,
    promotion: i8,
}

fn coordinates_from_index(index: usize) -> (usize, usize) {
    return (index % BOARD_SIZE, index / BOARD_SIZE)
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
        Move {
            from: 12,
            to: 28,
            promotion: 0,
        }.apply(Position::starting_position())
    );
}
