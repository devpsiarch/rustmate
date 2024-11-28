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


#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum SIDES {
    white,
    black,
}
