use core::f64;

use crate::SIDES;
use crate::get_move_enpassant;
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::move_type; 
use crate::Search;
use crate::evalu::{evaluate};
// petition to add nodes traversed and legal moves went thought in the seach process , for pretty
// stuff

#[allow(dead_code)] 
impl Search {
    // Searches for moves in a different way then for each color , just for expiremntation ik ik i
    // cant spell
    fn negamax(
        board:&mut Chessboard,atk:&AttackMasks,mut alpha:f64,beta:f64,
    ply:i32,depth:u32) -> f64 {

        // since we have define the evaluate as max for white and min for black
        // then we have to flip the signs 
        if depth == 0 {
            let eval = evaluate(*board);
            match board.side_to_move {
                SIDES::WHITE => {return eval;}
                SIDES::BLACK => {return -eval;}
            };
        }

        let mut generator = MoveGenerator::new(board, &atk);
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

        // move ordering
        generator.move_order();

        let mut best_score = -f64::INFINITY;
    
        let board_cpy = generator.board.clone();

        for i in 0..generator.moves.count {
            if !generator.make_move(generator.moves.list[i],move_type::ALL_MOVES){
                continue;
            }
    
            let score = -Self::negamax(
                &mut generator.board, atk, -beta, -alpha, ply + 1, depth - 1
            );

            generator.board.restore_board(board_cpy);

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                break;
            }
        }

        return best_score;
    }
    pub fn negamax_decision(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // This will stores the moves already made when we are searching
        let mut bestmove: Option<Move> = None;
        let mut bestscore = -f64::INFINITY;
            

        let mut generator = MoveGenerator::new(board, &atk);
        generator.generate_moves();

        let board_cpy = generator.board.clone();

        for i in 0..generator.moves.count {
            if !generator.make_move(generator.moves.list[i],move_type::ALL_MOVES){
                continue;
            }
            let score = -Self::negamax(
                &mut generator.board, atk, -f64::INFINITY,f64::INFINITY,1, depth-1
            );

            if score > bestscore {
                bestmove = Some(generator.moves.list[i]);
                bestscore = score;
            }

            generator.board.restore_board(board_cpy);
        }
        return bestmove;
    }
}
