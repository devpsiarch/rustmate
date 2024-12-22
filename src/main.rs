mod chessboard;
use chessboard::{Chessboard};

mod movegen;
use crate::movegen::MoveGenerator;
use crate::movegen::movecode::{Move,MoveMask};
use crate::movegen::movelist::{MoveList};
//i am using these here just for testing future me , take them down when everything is set
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::defs::{SIDES,Pieces,SQUARE};

/*
* Here use the crates that the main function does not need but you do for 
* debugging alr ? i dont wanna deal with a billion warnings
*/

use crate::movegen::{move_type};

//i will be running tests here untile everything is set and done
fn main() {
    // Here i define and init every "essential" <if you will> part of the engine
    let _m:Move = encode_move!(SQUARE::e4 as Move,SQUARE::e5 as Move,Pieces::P as Move,Pieces::Q as Move,1,0,0,1); 
    let mut chess = Chessboard::new();
    let mut test = attacks::AttackMasks::new();
    test.load_attacks_maps();
    chess.init_board();
    // From this point onwards the "chessboard" and the "attack maps" refreces belong to the
    // MoveGenerator
    
    let mut generator = MoveGenerator::new(&mut chess,&test);
    
    // Minupulating the board before generating the moves
    generator.board.spawn_piece(Pieces::B,SQUARE::c6);
    generator.board.pop_square(SQUARE::e2);
    generator.board.pop_square(SQUARE::a6);
    generator.board.pop_square(SQUARE::d7);
    generator.board.spawn_piece(Pieces::q,SQUARE::e3);

    generator.generate_moves();
  

    // testing goes here and only here
    generator.moves.print_all_moves();
    for i in 0..generator.moves.count {
        let copy = generator.board.clone();
        let mut input = String::new();
        if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) == true {
            copy.print_chessboard();
            generator.board.print_chessboard();
            std::io::stdin().read_line(&mut input);
        }else{
            {}
        }
        generator.board.restore_board(copy);
    }

    
}
