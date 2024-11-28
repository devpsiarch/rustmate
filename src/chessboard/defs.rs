//we define this to identify eche board in bitboards contained by Chessboard 
pub struct Pieces;
#[allow(non_upper_case_globals)]
impl Pieces {
    pub const P: usize = 0; // White Pawn
    pub const R: usize = 1; // White Rook
    pub const N: usize = 2; // White Knight
    pub const B: usize = 3; // White Bishop
    pub const Q: usize = 4; // White Queen
    pub const K: usize = 5; // White King

    pub const p: usize = 6; // Black Pawn
    pub const r: usize = 7; // Black Rook
    pub const n: usize = 8; // Black Knight
    pub const b: usize = 9; // Black Bishop
    pub const q: usize = 10; // Black Queen
    pub const k: usize = 11; // Black King
}
//we use these to describe the castling rights as a binary number 0 0 0 0 (K,Q,k,q)
pub struct Castle;
#[allow(non_upper_case_globals)]
impl Castle {
    pub const K: u8 = 1; 
    pub const Q: u8 = 2; 
    pub const k: u8 = 4; 
    pub const q: u8 = 8; 
}

pub const UNICODE_PIECES: [&str; 12] = [
    "♟︎", "♜" ,"♞" ,"♝", "♛", "♚",
    "♙", "♖" ,"♘", "♗", "♕", "♔"
];

pub const SQUARE_NAME: [&str; 65] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "NONE"
];

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum SIDES {
    white,
    black,
}
