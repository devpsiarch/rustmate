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
// wanted to use the max int value but it may cause overflow if not handled

pub mod minimax;
pub mod quitesearch;
pub mod negamax;

pub struct Search;
impl Search {
    // This methode will return the move (if found) and a status that indicates 
    // * what this move is ? (Checkmate/stalemate or just a regulate move)
    // * if finding move process failed for any reason (which is not supposed to happen btw)
    

    // TODO: should add the is_game_over function to stop the search
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // return Self::negamax_decision(board, atk, depth);
        return Self::minimax_decision(board, atk, depth);
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
