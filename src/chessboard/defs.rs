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
//This does the same job as the SIDES , i just use it to see the color thats all 
//yea i know am stupid shut up
pub struct Color;
#[allow(non_upper_case_globals)]
impl Color {
    pub const white: usize = 0; 
    pub const black: usize = 1;
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

pub struct SQUARE;
#[allow(non_upper_case_globals)]
//please excuse the disgusting code , i dont know a better way in rust
impl SQUARE {
    pub const a8: u8 = 0;
    pub const b8: u8 = 1;
    pub const c8: u8 = 2;
    pub const d8: u8 = 3;
    pub const e8: u8 = 4;
    pub const f8: u8 = 5;
    pub const g8: u8 = 6;
    pub const h8: u8 = 7;
    
    pub const a7: u8 = 8;
    pub const b7: u8 = 9;
    pub const c7: u8 = 10;
    pub const d7: u8 = 11;
    pub const e7: u8 = 12;
    pub const f7: u8 = 13;
    pub const g7: u8 = 14;
    pub const h7: u8 = 15;
    
    pub const a6: u8 = 16;
    pub const b6: u8 = 17;
    pub const c6: u8 = 18;
    pub const d6: u8 = 19;
    pub const e6: u8 = 20;
    pub const f6: u8 = 21;
    pub const g6: u8 = 22;
    pub const h6: u8 = 23;
    
    pub const a5: u8 = 24;
    pub const b5: u8 = 25;
    pub const c5: u8 = 26;
    pub const d5: u8 = 27;
    pub const e5: u8 = 28;
    pub const f5: u8 = 29;
    pub const g5: u8 = 30;
    pub const h5: u8 = 31;
    
    pub const a4: u8 = 32;
    pub const b4: u8 = 33;
    pub const c4: u8 = 34;
    pub const d4: u8 = 35;
    pub const e4: u8 = 36;
    pub const f4: u8 = 37;
    pub const g4: u8 = 38;
    pub const h4: u8 = 39;
    
    pub const a3: u8 = 40;
    pub const b3: u8 = 41;
    pub const c3: u8 = 42;
    pub const d3: u8 = 43;
    pub const e3: u8 = 44;
    pub const f3: u8 = 45;
    pub const g3: u8 = 46;
    pub const h3: u8 = 47;
    
    pub const a2: u8 = 48;
    pub const b2: u8 = 49;
    pub const c2: u8 = 50;
    pub const d2: u8 = 51;
    pub const e2: u8 = 52;
    pub const f2: u8 = 53;
    pub const g2: u8 = 54;
    pub const h2: u8 = 55;
    
    pub const a1: u8 = 56;
    pub const b1: u8 = 57;
    pub const c1: u8 = 58;
    pub const d1: u8 = 59;
    pub const e1: u8 = 60;
    pub const f1: u8 = 61;
    pub const g1: u8 = 62;
    pub const h1: u8 = 63;
}

pub const MAX_MOVE_RULE: u8 = 100;
pub const MAX_GAME_MOVES: u16 = 2048;
#[derive(Clone)]
#[allow(non_camel_case_types)]

pub enum SIDES {
    white,
    black,
}
