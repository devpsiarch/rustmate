pub mod defs;
use defs::{SIDES};
pub mod bitboard;
use bitboard::{Bitboard};
mod fen;
pub mod attacks;


#[derive(Clone)]
pub struct Chessboard {
    bitboards : [Bitboard;12],
    side_to_mode : SIDES,
    castling_rights : u8,
    en_passant : u8,
    half_move_clock : u8,
    move_count : u16,
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI , 
    pub fn new() -> Self {
        Self {
            bitboards : [0;12], 
            side_to_mode : SIDES::white,
            castling_rights : 0,
            en_passant : 64,            //i define 64 as none as in no en passant are availble
            half_move_clock : 0,
            move_count : 1,
        } 
    }
    //this may have to be set to private later on
    pub fn reset(&mut self) {
        self.bitboards = [0;12];
        self.side_to_mode = SIDES::white;
        self.castling_rights = 0;
        self.en_passant = 64;
        self.half_move_clock = 0;
        self.move_count = 1;
    } 
}
