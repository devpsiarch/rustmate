mod chessboard;
use chessboard::{Chessboard};
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{Pieces,SQUARE};


mod movegen;
use crate::movegen::movecode::{Move,MoveMask};
use crate::movegen::movelist::{MoveList};
//i am using these here just for testing future me , take them down when everything is set
use crate::movegen::perft::{perft_driver};

mod comm;
use crate::comm::uci;
//use crate::movegen::MoveGenerator;
/*
* Here use the crates that the main function does not need but you do for 
* debugging alr ? i dont wanna deal with a billion warnings
*/

use crate::movegen::{move_type};
use std::time::Instant;
//i will be running tests here untile everything is set and done
fn main() {
    match uci() {
        Ok(()) => println!("UCI protocol session ended with success."),
        Err(code) => println!("UCI protocol session exited with error code {code}") ,
    }
    return ;
    // init the ATTACK tables , sooner we will replace this with an instance that will do everything
    let mut ATTACK_TABLE = attacks::AttackMasks::new();
    ATTACK_TABLE.load_attacks_maps();

    let mut chess = Chessboard::new();
    chess.init_board();
    chess.print_chessboard(); 
    let start = Instant::now(); 
    println!("Moves found: {}",perft_driver(&mut chess,&ATTACK_TABLE, 6));
    println!("Time taken: {:.2?}",start.elapsed());
    return;
}
