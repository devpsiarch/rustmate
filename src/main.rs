mod chessboard;
use chessboard::{Chessboard};

mod movegen;
use crate::movegen::movecode::{Move,MoveMask};
use crate::movegen::movelist::{MoveList};
//i am using these here just for testing future me , take them down when everything is set
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{Pieces,SQUARE};

use crate::movegen::perft::{perft_driver};
//use crate::movegen::MoveGenerator;
/*
* Here use the crates that the main function does not need but you do for 
* debugging alr ? i dont wanna deal with a billion warnings
*/

use crate::movegen::{move_type};
use std::time::Instant;
//i will be running tests here untile everything is set and done
fn main() {
    // Here i define and init every "essential" <if you will> part of the engine
    let _m:Move = encode_move!(SQUARE::e4 as Move,SQUARE::e5 as Move,Pieces::P as Move,Pieces::Q as Move,1,0,0,1); 
    let mut chess = Chessboard::new();
    let mut test = attacks::AttackMasks::new();
    test.load_attacks_maps();
    chess.init_board();
    chess.print_chessboard(); 
    let start = Instant::now(); 
    println!("Moves found: {}",perft_driver(&mut chess,&test, 6));
    println!("Time taken: {:.2?}",start.elapsed());
    return;
}
