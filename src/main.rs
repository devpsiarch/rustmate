mod chessboard;
use chessboard::{Chessboard};
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{FenPositions};


mod movegen;
use crate::movegen::movecode::{Move,MoveMask};
use crate::movegen::movelist::{MoveList};
//i am using these here just for testing future me , take them down when everything is set
use crate::movegen::perft::{perft_driver};

mod comm;
use crate::comm::uci;

mod search;

mod evalu;
use crate::evalu::evaluate;
//use crate::movegen::MoveGenerator;
/*
* Here use the crates that the main function does not need but you do for 
* debugging alr ? i dont wanna deal with a billion warnings
*/

use crate::movegen::{move_type};
use std::time::Instant;

use crate::search::{Search,INF};
//i will be running tests here untile everything is set and done
fn main() {
     // init the ATTACK tables , sooner we will replace this with an instance that will do everything
    let mut attacks = attacks::AttackMasks::new();
    attacks.load_attacks_maps();
    let mut chess = Chessboard::new();   

    let dev = true;

    // Then we are developing the engine
    if dev == true {
        chess.init_board("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        chess.print_chessboard();
        println!("the value of this board : {}",evaluate(chess.clone())); 
        let start = Instant::now(); 
        println!("best eval: {}",Search::minimax_alpha_beta(&mut chess.clone(),&attacks,6,-INF,INF,chess.side_to_move.clone()));
        println!("Time taken: {:.2?}",start.elapsed());
        return ;
        let start = Instant::now(); 
        println!("Moves found: {}",perft_driver(&mut chess,&attacks, 5));
        println!("Time taken: {:.2?}",start.elapsed());
    }
    // Then we are working on the UCI
    else{
        match uci(&mut chess,&attacks) {
            Ok(()) => println!("UCI protocol session ended with success."),
            Err(code) => println!("UCI protocol session exited with error code {code}") ,
        }
    } 

    return;
}
