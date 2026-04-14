use crate::SIDES;
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
        board:&mut Chessboard,atk:&AttackMasks,mut alpha:i32,beta:i32,
    ply:i32,depth:u32) -> i32 {

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

        if generator.stale_mate() {
            return 0;
        }

        let mut best_score = i32::MIN;
    
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
        let mut bestmove:Move = 0;
        let mut bestscore = i32::MIN;
            

        let mut generator = MoveGenerator::new(board, &atk);
        generator.generate_moves();

        let board_cpy = generator.board.clone();

        for i in 0..generator.moves.count {
            if !generator.make_move(generator.moves.list[i],move_type::ALL_MOVES){
                continue;
            }
            let score = -Self::negamax(
                &mut generator.board, atk, i32::MIN,i32::MAX,1, depth
            );

            if score > bestscore {
                bestmove = generator.moves.list[i];
                bestscore = score;
            }

            generator.board.restore_board(board_cpy);
        }
        return Some(bestmove);
    }
}
