use crate::movegen::{MoveGenerator,MakeMoveError}; 
use crate::chessboard::{Chessboard};
use crate::chessboard::attacks::{AttackMasks};
use crate::move_type; 

/*
 * This is not a part of the engine but needed to test if the move generation and the core engine
 * in working correctly withou any mistakes .
*/


pub fn perft_driver(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    // Creating a generator object
    let mut generator = MoveGenerator::new(board,&atk);  
    generator.generate_moves();
    let mut nodes:u64 = 0;
    let copy = generator.board.clone();
    for i in 0..generator.moves.count {

        let _killed = match generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) {
            Ok(maybe_killed_piece) => maybe_killed_piece,
            Err(MakeMoveError::CaptureConflict) | Err(MakeMoveError::Illegal) => {
                continue;
            }
        };

        nodes += perft_driver(&mut generator.board,&generator.attacks, depth - 1);

        generator.board.restore_board(copy);
    }
    nodes
}
