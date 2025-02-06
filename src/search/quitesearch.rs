use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::evalu::{evaluate};
use crate::move_type; 
use crate::Search;
// We will consider this as the infinity value

impl Search {
    // minimax algorithm
    // This already takes the best move found so far , it may change or not if found some better
    pub fn quite_search(
        board:&mut Chessboard,atk:&AttackMasks,mut alpha:i32,mut beta:i32,ply:&mut i32)
        -> i32 {
        let eval = evaluate(*board);
        if eval >= beta {
            return beta;
        }
        if alpha < eval {
            alpha = eval;
        } 

        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
       
        let copy = generator.board.clone();
        for i in 0..generator.moves.count {
            *ply += 1;
            if !generator.make_move(generator.moves.list[i],move_type::CAPTURE_MOVE) {
                *ply -=1;
                continue;
            }
            let mut score = -Self::quite_search(&mut generator.board,atk,-beta,-alpha,ply); 
            *ply -= 1;
            generator.board.restore_board(copy);
            if score >= beta {
                return beta;
            }        
            if score > alpha {
                alpha = score;
            }
        }
        return alpha;
    }
}
