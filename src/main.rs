mod chessboard;
use chessboard::{Chessboard};

//i am using these here just for testing future me , take them down when everything is set
use crate::chessboard::bitboard;
use crate::chessboard::attacks;
use crate::chessboard::defs;
use crate::chessboard::magic;
use crate::bitboard::Bitboard;

//i will be running tests here untile everything is set and done
fn main() {
    //let mut chess = Chessboard::new();
    //chess.parse_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
    //chess.print_chessboard();   
    //chess.parse_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9");
    //chess.print_chessboard();
    //chess.parse_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
    //chess.print_chessboard();
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
    set_bit!(occ,defs::SQUARE::d5);
    set_bit!(occ,defs::SQUARE::e6);
    set_bit!(occ,defs::SQUARE::g5);
    set_bit!(occ,defs::SQUARE::a2);
    let mut test = attacks::get_rook_attack_otfmask(occ,defs::SQUARE::a8);
    bitboard::print_bitboard(&test);
    //pop_bit!(test,defs::SQUARE::e5);
    //bitboard::print_bitboard(&test);
    //let c = bitboard::bit_count(test);
    //let ksb = bitboard::get_lsb(test);
    //println!("{}",c);
    //println!("{}",ksb);

}
