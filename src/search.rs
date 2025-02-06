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
pub mod quitesearch;
pub mod negamax;
pub struct Search;
impl Search {
    // This methode will return the move (if found) and a status that indicates 
    // * what this move is ? (Checkmate/stalemate or just a regulate move)
    // * if finding move process failed for any reason (which is not supposed to happen btw)
    pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // This will stores the moves already made when we are searching
        let mut ply:i32 = 0; 
        let mut bestmove:Move = 0;
        // Uncomment and comment others is you want to use the negemax algo
        let _eval = Self::negamax(board,atk,-INF,INF,&mut ply,&mut bestmove,depth);

        // Uncomment and commnet others if you wanna use my version of the minimax algo 
        //let (_eval,bestmove) = Self::minimax_alpha_beta(board,atk,depth,-INF,INF,board.side_to_move,&mut ply); 
        
        // This should always be here dont change , the handler expects a move response
        // More move codes will be added later to experess which side won/draw.
        if bestmove == 0 {
            return None;
        }
        else {
            Some(bestmove)
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
