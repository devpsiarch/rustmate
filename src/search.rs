/* 
* We seach for moves in this module , well do all sort of ways to do so ...
* 1- random move generator <baby step>
* */
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::chessboard::defs::{SIDES,Pieces};
use crate::evalu::{evaluate};
use rand::{thread_rng, Rng};
use std::cmp;
use crate::move_type; 
// We will consider this as the infinity value
pub const INF:i32 = 1_000_000; 
// wanted to use the max int value but it may cause overflow if not handled

pub struct Search;
impl Search {
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        let (_best_eval,bestmove) = Self::minimax_alpha_beta(board,atk,depth,-INF,INF,board.side_to_move);
        //Self::random_move(board,atk)
        bestmove
    }
    
    // minimax algorithm
    // we return instead 2 values , the bestmove and its eval
    pub fn minimax_alpha_beta(
        board:&mut Chessboard,atk:&AttackMasks,depth:u32,mut alpha:i32,mut beta:i32,color:SIDES)
        -> (i32,Option<Move>) {
        // We also need to consider the game ending
        if depth == 0 {
            return (evaluate(*board),None);
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        
        // checking if the game ended 
        if generator.check_mate() {
            return (-(INF-1000),None);  // i think sending Some(mate) is better to know what the
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
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    // we dont care about the best move the oposite side might make
                    let (eval,_) = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::BLACK);
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
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    // again we dont care about the best moves the opponante migh make
                    let (eval,_) = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::WHITE);
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
    // This is the move searchers that will be here
    #[allow(dead_code)] 
    pub fn random_move(board:&mut Chessboard,atk:&AttackMasks) -> Option<Move> {
        let mut rng = thread_rng();
        let mut generator = MoveGenerator::new(board,atk);
        generator.generate_moves();
        Some(generator.moves.list[rng.gen_range(0..generator.moves.count) as usize])
    }
}
