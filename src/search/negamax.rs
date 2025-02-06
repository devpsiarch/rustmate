use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::move_type; 
use crate::Search;
use crate::search::minimax::INF;
use crate::evalu::{evaluate};
// petition to add nodes traversed and legal moves went thought in the seach process , for pretty
// stuff

impl Search {
    // Searches for moves in a different way then for each color , just for expiremntation ik ik i
    // cant spell
    pub fn negamax(
        board:&mut Chessboard,atk:&AttackMasks,mut alpha:i32,mut beta:i32,
    ply:&mut i32,best_move_found:&mut Move,depth:u32) -> i32 {
        if depth == 0 {
            // quite search here
            return evaluate(*board);
            //return Self::quite_search(board,atk,alpha,beta,ply);
        }
        // check for checkmates and stalemates
        let mut best_move_sofar:Move = 0;
        let saved_alpha = alpha;

        // generating the moves
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        
        // checking if the game ended 
        if generator.check_mate() {
            return -INF+*ply;  // i think sending Some(mate) is better to know what the
            // hell happended 
        }
        if generator.stale_mate() {
            return 0;
        }

        // make copy 
        let copy = generator.board.clone();
        for i in 0..generator.moves.count {
            *ply += 1;
            if !generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                *ply -=1;
                continue;
            }
            let score = -Self::negamax(&mut generator.board,atk,-beta,-alpha,ply,best_move_found,depth-1); 
            *ply -= 1;
            generator.board.restore_board(copy);
            if score >= beta {
                return beta;
            }        
            if score > alpha {
                alpha = score;
                if *ply == 0 {
                    best_move_sofar = generator.moves.list[i];
                }
            }
        }
        if saved_alpha != alpha {
            *best_move_found = best_move_sofar;
        }
        return alpha;
    }
}
