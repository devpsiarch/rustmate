pub mod defs;
use defs::{Pieces,COLOR,SQUARE_NAME,SIDES,SQUARE,STARTING_POSITION,CMK_POSITION,TRICKY_POSITION};
pub mod bitboard;
use bitboard::{Bitboard};
use crate::set_bit;
use crate::get_bit;
use crate::pop_bit;
mod fen;
pub mod attacks;
pub mod magic;
pub mod atkgen;


#[derive(Clone)]
pub struct Chessboard {
    pub bitboards : [Bitboard;12],          //for each piece and diff color
    pub side_to_move : SIDES,               // its an enum carfull
    pub occupencies : [Bitboard;3],         // one for black , black and both
    pub castling_rights : u8,               // binary rep each bit encodes for a right
    pub en_passant : u8,                    // number from 0 to 64 for all squares and a NONE option
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
            side_to_move : SIDES::WHITE,
            occupencies:[0;3],
            castling_rights : 0,
            en_passant : SQUARE::NO_SQUARE,            //i define 64 as none as in no en passant are availble
            half_move_clock : 0,
            move_count : 1,
        } 
    }
    // If init function of init_board is still just fen parsing then it needs to go 
    pub fn init_board(&mut self) {
        match self.parse_fen(TRICKY_POSITION) {
            Ok(()) => {}
            Err(error_code) => panic!("failed to parse fen from init_board : code {error_code}")
        }
    }
    // This just spawns a pices for a side as long as there is not pre existing piece there with NO REGARD to any chess rule 
    // AGAIN THIS DOES NOT PRODUCE MOVES , THIS IS USED FOR TESTING
    pub fn spawn_piece(&mut self,piece:usize,color:SIDES,square:u8) {
        if get_bit!(self.occupencies[COLOR::BOTH],square) != 1 {
            set_bit!(self.bitboards[piece],square);
            set_bit!(self.occupencies[COLOR::BOTH],square);
            match color {
                SIDES::WHITE => set_bit!(self.occupencies[COLOR::w],square),
                SIDES::BLACK => set_bit!(self.occupencies[COLOR::b],square),
            }
        }else{
            match color {
                SIDES::WHITE => println!("Spawning {} side white on {} is not permitable",piece,SQUARE_NAME[square as usize]),
                SIDES::BLACK => println!("Spawning {} side black on {} is not permitable",piece,SQUARE_NAME[square as usize]),
            }
        }
    }
    // This methode is the complement for the methode above and used only for testing 
    pub fn pop_square(&mut self,square:u8) {
        // Erasses a piece for the Chessboard object disregarding any rules of implications that
        // may cause , THIS IS NOT A PART OF MAKING A MOVE 
        for i in Pieces::P..=Pieces::k {
            println!("{}",i);
            pop_bit!(self.bitboards[i as usize],square);
        }
        for i in COLOR::w..=COLOR::BOTH {
            pop_bit!(self.occupencies[i as usize],square);
            println!("{}",i);
        }
    }
    //this may have to be set to private later on
    pub fn reset(&mut self) {
        self.bitboards = [0;12];
        self.side_to_move = SIDES::WHITE;
        self.occupencies = [0;3];
        self.castling_rights = 0;
        self.en_passant = 64;
        self.half_move_clock = 0;
        self.move_count = 1;
    } 
}
