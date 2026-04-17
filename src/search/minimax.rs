use crate::evalu::evaluate;
use crate::movegen::MakeMoveError;
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::chessboard::defs::{SIDES};
use core::f64;
use crate::move_type; 
use crate::Search;

impl Search {
    // minimax algorithm with alpha beta pruning
    fn minimax_alpha_beta(
        board:&mut Chessboard,atk:&AttackMasks,depth:u32,mut alpha:f64,mut beta:f64,color:SIDES,ply:i32)
        -> f64 {
        if depth == 0 {
            return evaluate(*board); 
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
 
        if generator.check_mate() {
            match generator.board.side_to_move {
                SIDES::WHITE => {return -100_000.0}
                SIDES::BLACK => {return 100_1000.0}
            }
        }

        if generator.stale_mate() {
            return 0.0;
        }

        generator.move_order();

        match color {
            // White wants to maximize the evaluation 
            SIDES::WHITE => {
                let mut maxeval = -f64::INFINITY;
                for i in 0..generator.moves.count {

                    let packet = match generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                        Ok(maybe_killed_piece) => maybe_killed_piece,
                        Err(MakeMoveError::CaptureConflict) | Err(MakeMoveError::Illegal) => {
                            continue;
                        }
                    };

                    // we dont care about the best move the oposite side might make
                    let eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::BLACK,ply+1);
                    // a better move was found
                    if eval > maxeval {
                        maxeval = eval;
                    }
                    alpha = eval.max(alpha);

                    match generator.unmake_move(generator.moves.list[i],packet) {
                        Ok(()) => (),
                        Err(_) => panic!("[SEARCH]: unable to unmake move during search."),
                    }

                    if alpha >= beta {
                        break;
                    }
                }
                maxeval
            }
            // Black wants to minimize the evaluation
            SIDES::BLACK => {
                let mut minval = f64::INFINITY;
                for i in 0..generator.moves.count {

                        let packet = match generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                            Ok(maybe_killed_piece) => maybe_killed_piece,
                            Err(MakeMoveError::CaptureConflict) | Err(MakeMoveError::Illegal) => {
                                continue;
                            }
                        };

                        // again we dont care about the best moves the opponante migh make
                        let eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::WHITE,ply+1);
                        if eval < minval {
                            minval = eval;
                        }
                        beta = eval.max(beta); 
                        match generator.unmake_move(generator.moves.list[i],packet) {
                            Ok(()) => (),
                            Err(_) => panic!("[SEARCH]: unable to unmake move during search."),
                        }

                        if alpha >= beta {
                            break;
                        }
                }
                minval
            }
        }
    }
    pub fn minimax_decision(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // This will stores the moves already made when we are searching
        let mut bestmove:Option<Move> = None;
        let mut bestscore = match board.side_to_move {
            SIDES::WHITE => -f64::INFINITY,
            SIDES::BLACK => f64::INFINITY,
        };
 
        let mut alpha = -f64::INFINITY;
        let mut beta = f64::INFINITY;


        let mut generator = MoveGenerator::new(board, &atk);
        generator.generate_moves();

        for i in 0..generator.moves.count {
            let packet = match generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                Ok(maybe_killed_piece) => maybe_killed_piece,
                Err(MakeMoveError::CaptureConflict) | Err(MakeMoveError::Illegal) => {
                    continue;
                }
            };

            let score = Self::minimax_alpha_beta(
                generator.board, atk, depth-1, -f64::INFINITY, f64::INFINITY, generator.board.side_to_move, 1
            );

            match generator.unmake_move(generator.moves.list[i],packet) {
                Ok(()) => (),
                Err(_) => panic!("[SEARCH]: unable to unmake move during search."),
            }

            match generator.board.side_to_move {
                SIDES::WHITE => {
                    if score > bestscore {
                        bestmove = Some(generator.moves.list[i]);
                        bestscore = score;
                    }
                    alpha = alpha.max(score);
                }
                SIDES::BLACK => {
                    if score < bestscore {
                        bestmove = Some(generator.moves.list[i]);
                        bestscore = score;
                    }
                    beta = beta.max(score);
                }
            }



            if alpha >= beta {
                break;
            }
        }
        return bestmove;
    }
}
