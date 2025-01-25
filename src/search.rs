/* 
* We seach for moves in this module , well do all sort of ways to do so ...
* 1- random move generator <baby step>
* */
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use rand::{thread_rng, Rng};
// We will consider this as the infinity value
pub const INF:i32 = 50_000; 
// wanted to use the max int value but it may cause overflow if not handled
pub mod minimax;

pub struct Search;
impl Search {
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // This will stores the moves already made when we are searching
        let mut ply:i32 = 0; 
        let (_best_eval,bestmove) = Self::minimax_alpha_beta(board,atk,depth,-INF,INF,board.side_to_move,&mut ply);
        //Self::random_move(board,atk)
        bestmove
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
