mod chessboard;
use chessboard::{Chessboard};

//i am using these here just for testing future me , take them down when everything is set
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::bitboard::Bitboard;
use crate::defs::SLIDER;

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
    chess.parse_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
    chess.print_chessboard();   
    chess.parse_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9");
    chess.print_chessboard();
    chess.parse_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
    chess.print_chessboard();
    //bitboard::print_bitboard(&attacks::NOT_H_FILE);
    //let mut test = attacks::get_pawn_attack_mask(defs::SQUARE::h4,defs::SIDES::black);
    //bitboard::print_bitboard(&test);
    //test = attacks::get_king_attack_mask(defs::SQUARE::h4);
    //bitboard::print_bitboard(&test);
    //test = attacks::get_knight_attack_mask(defs::SQUARE::h4);
    //bitboard::print_bitboard(&test);
    let mut test = attacks::AttackMasks::new();
    test.load_attacks_maps();
    //for i in 0..64 {
    //    bitboard::print_bitboard(&test.knight_attack_masks[i]);
    //}
    let mut occ: Bitboard = 0;
    set_bit!(occ,defs::SQUARE::c2);
    set_bit!(occ,defs::SQUARE::c6);
    set_bit!(occ,defs::SQUARE::b1);
    set_bit!(occ,defs::SQUARE::a1);
    set_bit!(occ,defs::SQUARE::h2);
    set_bit!(occ,defs::SQUARE::g6);
    set_bit!(occ,defs::SQUARE::c1);
    set_bit!(occ,defs::SQUARE::d1);
    for i in 0..64 {
        let map = test.lookup_slider(SLIDER::bishop,occ,i);
        let map2 = attacks::get_bishop_attack_otfmask(occ,i);
        if map != map2 {
            println!("maps do not match");
        } 
    } 
    //bitboard::print_bitboard(&occ);
    //bitboard::print_bitboard(&map);
    //let c = bitboard::bit_count(test);
    //let ksb = bitboard::get_lsb(test);
    //println!("{}",c);
    //println!("{}",ksb);

}
