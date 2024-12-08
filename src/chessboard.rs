pub mod defs;
use defs::{SIDES,SQUARE,STARTING_POSITION};
pub mod bitboard;
use bitboard::{Bitboard};
mod fen;
pub mod attacks;
pub mod magic;
pub mod atkgen;


#[derive(Clone)]
pub struct Chessboard {
    pub bitboards : [Bitboard;12],          //for each piece and diff color
    side_to_mode : SIDES,               // its an enum carfull
    pub occupencies : [Bitboard;3],         // one for black , black and both
    castling_rights : u8,               // binary rep each bit encodes for a right
    en_passant : u8,                    // number from 0 to 64 for all squares and a NONE option
    half_move_clock : u8,               // regular counter 
    move_count : u16,                  // same here 
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI ,
    // This creates a kind of "NULL" board that is illegal and loading pieces would make it so
    pub fn new() -> Self {
        Self {
            bitboards : [0;12], 
            side_to_mode : SIDES::WHITE,
            occupencies:[0;3],
            castling_rights : 0,
            en_passant : SQUARE::NO_SQUARE,            //i define 64 as none as in no en passant are availble
            half_move_clock : 0,
            move_count : 1,
        } 
    }
    // If init function of init_board is still just fen parsing then it needs to go 
    pub fn init_board(&mut self) {
        match self.parse_fen(STARTING_POSITION) {
            Ok(()) => {}
            Err(error_code) => panic!("failed to parse fen from init_board : code {error_code}")
        }
    }
    //this may have to be set to private later on
    pub fn reset(&mut self) {
        self.bitboards = [0;12];
        self.side_to_mode = SIDES::WHITE;
        self.occupencies = [0;3];
        self.castling_rights = 0;
        self.en_passant = 64;
        self.half_move_clock = 0;
        self.move_count = 1;
    } 
}
