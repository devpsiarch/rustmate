use core::panic;

use crate::evalu::defs::PIECES_COST;
use crate::movegen::{MoveGenerator,MakeMoveError};
use crate::search::quitesearch;
use crate::{Chessboard, get_move_capture, get_move_enpassant};
use crate::attacks::AttackMasks;
use crate::evalu::{evaluate};
use crate::move_type; 
use crate::Search;
// We will consider this as the infinity value

const DELTA_MARGIN: f64 = 400.0;

#[allow(dead_code)] 
impl Search {
    // minimax algorithm
    // This already takes the best move found so far , it may change or not if found some better
    pub fn quite_search(board:&mut Chessboard,atk:&AttackMasks,mut alpha:f64,beta:f64,ply:i32) -> f64 {
        let static_eval = evaluate(*board);

        let mut best_value = static_eval;
        if best_value >= beta {
            return best_value;
        }
        if best_value > alpha {
            alpha = best_value;
        }


        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        
        if generator.stale_mate() {
            return 0.0;
        }

        generator.move_order();

        for i in 0..generator.moves.count {

            if let Ok(packet) = generator.make_move(generator.moves.list[i],move_type::CAPTURE_MOVE) {
                
                if let Some(p) = packet.captured_piece {
                    if PIECES_COST[p] + static_eval + DELTA_MARGIN < alpha {
                    if let Err(_) = generator.unmake_move(generator.moves.list[i], packet) {
                        panic!("[SEARCH]: failed to unmake move in quite_search!");
                    }
                        continue;
                    }
                }

                let score = -Search::quite_search(generator.board, atk, -beta, -alpha, ply+1);
                
                if let Err(_) = generator.unmake_move(generator.moves.list[i], packet) {
                    panic!("[SEARCH]: failed to unmake move in quite_search!");
                }
                
                if score >= beta {
                    return score;
                }
                if score > best_value {
                    best_value = score;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }
        return best_value;
    }
}
