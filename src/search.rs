/* 
* We seach for moves in this module , well do all sort of ways to do so ...
* 1- random move generator <baby step>
* */
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::chessboard::defs::{SIDES};
use crate::evalu::{evaluate};
use rand::{thread_rng, Rng};
use std::cmp;
use crate::move_type; 
// We will consider this as the infinity value
pub const INF:i32 = 1_000_000; 
// wanted to use the max int value but it may cause overflow if not handled

pub struct Search;
impl Search {
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Move{
        let best_eval = Self::minimax_alpha_beta(board,atk,depth,-INF,INF,board.side_to_move);
        //Self::random_move(board,atk)
        0
    }
    
    // minimax algorithm
    pub fn minimax_alpha_beta(board:&mut Chessboard,atk:&AttackMasks,depth:u32,mut alpha:i32,mut beta:i32,color:SIDES) -> i32 {
        // We also need to consider the game ending
        if depth == 0 {
            return evaluate(*board);
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        // White wants to maximize the evaluation 
        if color == SIDES::WHITE {
            let mut maxeval = -INF;
            let mut eval:i32 = 0;
            let copy = generator.board.clone();
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::BLACK);
                    maxeval = cmp::max(eval,maxeval);
                    alpha = cmp::max(eval,alpha);
                    generator.board.restore_board(copy);
                    // pruning if we are sure we wont find better moves
                    if beta <= alpha{
                        break;
                    }
                }
            }
            maxeval
        }
        // Black wants to minimize the evaluation
        else{
            let mut minval = INF;
            let mut eval:i32 = 0;
            let copy = generator.board.clone();
            for i in 0..generator.moves.count {
                if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                    eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::WHITE);
                    minval = cmp::min(eval,minval); 
                    beta = cmp::min(eval,beta); 
                    generator.board.restore_board(copy);
                    // pruning
                    if beta <= alpha {
                        break;
                    }
                }
            }
            minval
        }
    }
       
    // This is the move searchers that will be here
    pub fn random_move(board:&mut Chessboard,atk:&AttackMasks) -> Move {
        let mut rng = thread_rng();
        let mut generator = MoveGenerator::new(board,atk);
        generator.generate_moves();
        generator.moves.list[rng.gen_range(0..generator.moves.count) as usize]
    }
}
