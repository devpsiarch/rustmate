use crate::evalu::evaluate;
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;
use crate::chessboard::defs::{SIDES};
use std::{cmp, i32};
use crate::move_type; 
use crate::Search;

impl Search {
    // minimax algorithm with alpha beta pruning
    fn minimax_alpha_beta(
        board:&mut Chessboard,atk:&AttackMasks,depth:u32,mut alpha:i32,mut beta:i32,color:SIDES,ply:i32)
        -> i32 {
        if depth == 0 {
            return evaluate(*board); 
        }
        // Creating a generator object
        let mut generator = MoveGenerator::new(board,&atk);  
        generator.generate_moves();
        
        if generator.stale_mate() {
            return 0;
        }

        match color {
            // White wants to maximize the evaluation 
            SIDES::WHITE => {
                let mut maxeval = i32::MIN;
                let copy = generator.board.clone();
                for i in 0..generator.moves.count {
                    if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                        // we dont care about the best move the oposite side might make
                        let eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::BLACK,ply+1);
                        // a better move was found
                        if eval > maxeval {
                            maxeval = eval;
                        }
                        alpha = cmp::max(eval,alpha);
                        generator.board.restore_board(copy);

                        if alpha >= beta {
                            break;
                        }
                    }
                }
                maxeval
            }
            // Black wants to minimize the evaluation
            SIDES::BLACK => {
                let mut minval = i32::MAX;
                let copy = generator.board.clone();
                for i in 0..generator.moves.count {
                    if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
                        // again we dont care about the best moves the opponante migh make
                        let eval = Self::minimax_alpha_beta(&mut generator.board,atk,depth-1,alpha,beta,SIDES::WHITE,ply+1);
                        if eval < minval {
                            minval = eval;
                        }
                        beta = cmp::min(eval,beta); 
                        generator.board.restore_board(copy);

                        if alpha >= beta {
                            break;
                        }
                    }
                }
                minval
            }
        }
    }
    pub fn minimax_decision(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Option<Move>{
        // This will stores the moves already made when we are searching
        let mut bestmove:Move = 0;
        let mut bestscore = match board.side_to_move {
            SIDES::WHITE => i32::MIN,
            SIDES::BLACK => i32::MAX,
        };
 
        let mut alpha = i32::MIN;
        let mut beta =  i32::MAX;


        let mut generator = MoveGenerator::new(board, &atk);
        generator.generate_moves();

        let board_cpy = generator.board.clone();

        for i in 0..generator.moves.count {
            if !generator.make_move(generator.moves.list[i],move_type::ALL_MOVES){
                continue;
            }

            let score = Self::minimax_alpha_beta(
                generator.board, atk, depth-1, i32::MIN, i32::MAX, generator.board.side_to_move, 1
            );

            match board_cpy.side_to_move {
                SIDES::WHITE => {
                    if score > bestscore {
                        bestmove = generator.moves.list[i];
                        bestscore = score;
                    }
                    alpha = cmp::max(alpha,score);
                }
                SIDES::BLACK => {
                    if score < bestscore {
                        bestmove = generator.moves.list[i];
                        bestscore = score;
                    }
                    beta = cmp::min(beta,score);
                }
            }

            generator.board.restore_board(board_cpy);

            if alpha >= beta {
                break;
            }
        }
        return Some(bestmove);
    }
}
