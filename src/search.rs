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
const INF:i32 = std::i32::MAX; 

pub struct Search;
impl Search {
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Move{
        Self::random_move(board,atk)
    }
    
    // minimax algorithm
    pub fn minimax(board:&mut Chessboard,atk:&AttackMasks,depth:u32,color:SIDES) -> i32 {
        if depth == 0 || board.game_over() == true {
            return evaluate(*board);
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        // White wants to maximize the evaluation 
        if color == SIDES::WHITE {
            let mut maxeval = -INF;
            let mut eval:i32 = 0;
            for i in 0..generator.moves.count {
                let legal = generator.make_move(generator.moves.list[i],move_type::ALL_MOVES);
                if legal == true {
                    eval = Self::minimax(&mut generator.board,atk,depth-1,SIDES::BLACK);
                }
                maxeval = cmp::max(eval,maxeval); 
            }
            maxeval
        }
        // Black wants to minimize the evaluation
        else{
            let mut maxeval = INF;
            let mut eval:i32 = 0;
            for i in 0..generator.moves.count {
                let legal = generator.make_move(generator.moves.list[i],move_type::ALL_MOVES);
                if legal == true {
                    eval = Self::minimax(&mut generator.board,atk,depth-1,SIDES::WHITE);
                }
                maxeval = cmp::min(eval,maxeval); 
            }
            maxeval
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
