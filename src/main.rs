const BOARD_SIZE: usize = 8;

mod pieces {
    pub const NONE: i8 = 0;
    pub const PAWN: i8 = 1;
    pub const KNIGHT: i8 = 2;
    pub const BISHOP: i8 = 3;
    pub const ROOK: i8 = 4;
    pub const QUEEN: i8 = 5;
    pub const KING: i8 = 6;
}

#[derive(Debug, Copy, Clone)]
struct Position {
    board: [i8; 64],
    side_to_play: i8,
}

struct Move {
    from: usize,
    to: usize,
    promotion: i8,
}

fn to_coordinates(index: usize) -> (usize, usize) {
    return (index % BOARD_SIZE, index / BOARD_SIZE)
}

impl Position {
    fn starting_position() -> Position {
        Position {
            board: [
                pieces::ROOK, pieces::KNIGHT, pieces::BISHOP, pieces::QUEEN, pieces::KING, pieces::BISHOP, pieces::KNIGHT, pieces::ROOK,
                pieces::PAWN, pieces::PAWN, pieces::PAWN, pieces::PAWN, pieces::PAWN, pieces::PAWN, pieces::PAWN, pieces::PAWN,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                -pieces::PAWN, -pieces::PAWN, -pieces::PAWN, -pieces::PAWN, -pieces::PAWN, -pieces::PAWN, -pieces::PAWN, -pieces::PAWN,
                -pieces::ROOK, -pieces::KNIGHT, -pieces::BISHOP, -pieces::QUEEN, -pieces::KING, -pieces::BISHOP, -pieces::KNIGHT, -pieces::ROOK,
            ],
            side_to_play: 1,
        }
    }
}

impl Move {
    fn apply(&self, position: Position) -> Position {
        let mut position = position;

        let piece = position.board[self.from];
        position.board[self.from] = pieces::NONE;

        let (x, y) = to_coordinates(self.from);

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
