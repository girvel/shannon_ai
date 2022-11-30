mod piece {
    const NONE: i8 = 0;
    const PAWN: i8 = 1;
    const KNIGHT: i8 = 2;
    const BISHOP: i8 = 3;
    const ROOK: i8 = 4;
    const QUEEN: i8 = 5;
    const KING: i8 = 6;
}

struct Position {
    board: [i8; 64],
    side_to_play: i8,
}

struct Move {
    from: usize,
    to: usize,
    promotion: i8,
}

fn main() {
    println!("Hello, world!");
}
