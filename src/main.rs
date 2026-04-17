use core::panic;
use std::env;


mod chessboard;
use chessboard::{Chessboard};
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{FenPositions,SIDES};

mod movegen;
use crate::movegen::perft::perft_driver_undo;
use crate::movegen::{MoveGenerator,MakeMoveError};
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

enum EngineMode {
    UCI,
    PerfDriver,
    Custom
}

//i will be running tests here untile everything is set and done
#[allow(unreachable_code)]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");    // for debugging
    // init the ATTACK tables , sooner we will replace this with an instance that will do everything
    let mut attacks = attacks::AttackMasks::new();
    attacks.load_attacks_maps();
    let mut chess = Chessboard::new();   

    let mode = EngineMode::PerfDriver;

    match mode {
        EngineMode::UCI => {
            match uci(&mut chess,&attacks) {
                Ok(()) => println!("UCI protocol session ended with success."),
                Err(code) => println!("UCI protocol session exited with error code {code}") ,
            }
        }
        EngineMode::Custom => {
            chess.init_board(FenPositions::TRICKY_POSITION);
            
            let mut generator = MoveGenerator::new(&mut chess,&attacks);
            generator.generate_moves();
            
            generator.print_all_moves();

            println!("BEFORE THE MOVE");
            generator.board.print_chessboard();

            let i = 9;

            let killed = match generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                Ok(maybe_killed_piece) => maybe_killed_piece,
                Err(_) => {
                    panic!("failed to make the move");
                }
            };

            println!("AFTERT THE MOVE");
            generator.board.print_chessboard();

            match generator.unmake_move(generator.moves.list[i], killed){
                Ok(_) => {
                    println!("AFTER THE UNMAKE MOVE");
                    generator.board.print_chessboard();
                }
                Err(_) => {
                    panic!("unmake move failed....");
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
        }
        EngineMode::PerfDriver => {
            chess.init_board(FenPositions::POSITION_4);
            let depth = 5;
            println!("Perftest using copy retrace");
            let start = Instant::now();
            for i in 0..depth {
                println!("Moves found: {}",perft_driver(&mut chess,&attacks, i));
            }
            println!("Time taken: {:.2?}",start.elapsed());

            println!("Perftest using unmake_move");
            let start = Instant::now();
            for i in 0..depth {
                println!("Moves found: {}",perft_driver_undo(&mut chess,&attacks, i));
            }
            println!("Time taken: {:.2?}",start.elapsed());
        }
    }

    return;
}
