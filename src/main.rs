mod piece {
    pub const NONE: i8 = 0;
    pub const PAWN: i8 = 1;
    pub const KNIGHT: i8 = 2;
    pub const BISHOP: i8 = 3;
    pub const ROOK: i8 = 4;
    pub const QUEEN: i8 = 5;
    pub const KING: i8 = 6;
}

#[derive(Debug)]
struct Position {
    board: [i8; 64],
    side_to_play: i8,
}

struct Move {
    from: usize,
    to: usize,
    promotion: i8,
}

impl Position {
    fn starting_position() -> Position {
        Position {
            board: [
                piece::ROOK, piece::KNIGHT, piece::BISHOP, piece::QUEEN, piece::KING, piece::BISHOP, piece::KNIGHT, piece::ROOK,
                piece::PAWN, piece::PAWN, piece::PAWN, piece::PAWN, piece::PAWN, piece::PAWN, piece::PAWN, piece::PAWN,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                -piece::PAWN, -piece::PAWN, -piece::PAWN, -piece::PAWN, -piece::PAWN, -piece::PAWN, -piece::PAWN, -piece::PAWN,
                -piece::ROOK, -piece::KNIGHT, -piece::BISHOP, -piece::QUEEN, -piece::KING, -piece::BISHOP, -piece::KNIGHT, -piece::ROOK,
            ],
            side_to_play: 1,
        }
    }
}

fn main() {
    println!("{:?}", Position::starting_position());
}
