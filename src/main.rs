mod chessboard;
use chessboard::{Chessboard};

mod moves;
use crate::moves::MoveGenerator;
//i am using these here just for testing future me , take them down when everything is set
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::bitboard::{print_bitboard,Bitboard};
use crate::defs::{COLOR,SIDES,SLIDER,Pieces,SQUARE};
/*
* NOTES ::: future me 
* fix the loading / looking up the slider pieces attack tables for the rook cuz they be ass
* we have these options to fix the rook and bishop indecies {
* --> find magic numbers availbale online : Crafty and chess Wiki has some zip files
* --> using hashing : rust provides that and it wont be hard to use
*   }
* */


//i will be running tests here untile everything is set and done
fn main() {
    let mut chess = Chessboard::new();
    chess.init_board();
    let mut test = attacks::AttackMasks::new();
    test.load_attacks_maps();
    //chess.spawn_piece(Pieces::N,SIDES::WHITE,SQUARE::g8);
    //chess.spawn_piece(Pieces::N,SIDES::WHITE,SQUARE::b8);
    //chess.pop_square(SQUARE::e2);
    chess.print_chessboard();   
    let generator = MoveGenerator::new(&chess,&test);
    if generator.square_attacked(SIDES::BLACK,defs::SQUARE::f3) {
        println!("Attacked and working");
    }
    let mut atk = generator.attacked_squares(SIDES::BLACK);
    print_bitboard(&chess.occupencies[COLOR::BOTH]);
    generator.generate_pawn_moves();
    generator.generate_castle_moves();
    return;
    //for i in 0..64 {
    //    bitboard::print_bitboard(&test.knight_attack_masks[i]);
    //}
    let mut occ: Bitboard = 0;

    set_bit!(occ,defs::SQUARE::c4);
    set_bit!(occ,defs::SQUARE::e6);
    set_bit!(occ,defs::SQUARE::g2);
    set_bit!(occ,defs::SQUARE::f5);
    bitboard::print_bitboard(&occ);
    let map1 = test.lookup_slider(SLIDER::ROOK,occ,defs::SQUARE::e4);
    let map2 = test.lookup_slider(SLIDER::BISHOP,occ,defs::SQUARE::e4);
    let map4 = test.lookup_slider(SLIDER::QUEEN,occ,defs::SQUARE::e4);
    let map3 = map1 | map2;
    bitboard::print_bitboard(&map1);
    bitboard::print_bitboard(&map2);
    bitboard::print_bitboard(&map3);
    bitboard::print_bitboard(&map4);
    if map3 == map4 {
        println!("Hello queen");
    }
    //bitboard::print_bitboard(&occ);
    //bitboard::print_bitboard(&map);
    //let c = bitboard::bit_count(test);
    //let ksb = bitboard::get_lsb(test);
    //println!("{}",c);
    //println!("{}",ksb);

}
