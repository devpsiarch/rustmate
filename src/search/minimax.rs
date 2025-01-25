use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::chessboard::defs::{SIDES};
use crate::evalu::{evaluate};
use std::cmp;
use crate::move_type; 
use crate::Search;
// We will consider this as the infinity value
pub const INF:i32 = 50_000; 

impl Search {
    // minimax algorithm
    // we return instead 2 values , the bestmove and its eval
    pub fn minimax_alpha_beta(
        board:&mut Chessboard,atk:&AttackMasks,depth:u32,mut alpha:i32,mut beta:i32,color:SIDES,ply:&mut i32)
        -> (i32,Option<Move>) {
        // We also need to consider the game ending
        if depth == 0 {
            // Quinsearch also looks for checks as well as captures
            return (evaluate(*board),None);
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        
        // checking if the game ended 
        if generator.check_mate() {
            return (-INF+*ply,None);  // i think sending Some(mate) is better to know what the
            // hell happended 
        }
        if generator.stale_mate() {
            return (0,None);
        }
        // White wants to maximize the evaluation 
        if color == SIDES::WHITE {
            let mut maxeval = -INF;
            let mut bestmove = None;
            let copy = generator.board.clone();
            *ply += 1;
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    // we dont care about the best move the oposite side might make
                    let (eval,_) = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::BLACK,ply);
                    *ply -= 1;
                    // a better move was found
                    if eval > maxeval {
                        maxeval = eval;
                        bestmove = Some(generator.moves.list[i]);
                    }
                    alpha = cmp::max(eval,alpha);
                    generator.board.restore_board(copy);
                    // pruning if we are sure we wont find better moves
                    if beta <= alpha{
                        break;
                    }
                }
            }
            (maxeval,bestmove)
        }
        // Black wants to minimize the evaluation
        else{
            let mut minval = INF;
            let mut bestmove = None;
            let copy = generator.board.clone();
            *ply += 1;
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    // again we dont care about the best moves the opponante migh make
                    let (eval,_) = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::WHITE,ply);
                    *ply -= 1;
                    if eval < minval {
                        minval = eval;
                        bestmove = Some(generator.moves.list[i]);
                    }
                    beta = cmp::min(eval,beta); 
                    generator.board.restore_board(copy);
                    // pruning
                    if beta <= alpha {
                        break;
                    }
                }
            }
            (minval,bestmove)
        }
    }
}
