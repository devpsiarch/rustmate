use std::env;


mod chessboard;
use chessboard::{Chessboard};
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{FenPositions,SIDES};

mod movegen;
use crate::movegen::MoveGenerator;
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

use crate::search::{Search};
//i will be running tests here untile everything is set and done
#[allow(unreachable_code)]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");    // for debugging
    // init the ATTACK tables , sooner we will replace this with an instance that will do everything
    let mut attacks = attacks::AttackMasks::new();
    attacks.load_attacks_maps();
    let mut chess = Chessboard::new();   

    let dev = false;

    // Then we are developing the engine
    if dev == true {
        chess.init_board(FenPositions::KILLER_POSITION);
        
        let mut generator = MoveGenerator::new(&mut chess,&attacks);
        generator.generate_moves();
        
        generator.print_all_moves();
        for i in 0..generator.moves.count {
            let copy = generator.board.clone();
            if generator.make_move(generator.moves.list[i],move_type::CAPTURE_MOVE) {
                println!("atk move here");
                generator.board.restore_board(copy);
            }

        }
        return;
        println!("the value of this board : {}",evaluate(chess.clone())); 
        let start = Instant::now(); 
        let bestmove = Search::search_move(&mut chess.clone(),&attacks,6);
        let mut accual_best:Move;
        match bestmove {
            Some(x) => accual_best = x,
            None => accual_best = 0,
        }
        println!("best move: {}",accual_best);
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
