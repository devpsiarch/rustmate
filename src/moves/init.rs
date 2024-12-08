/*
* Well define some essential methodes here*/
use crate::chessboard::attacks::AttackMasks;
use crate::chessboard::{Chessboard};
use crate::chessboard::defs::{SIDES,SLIDER,Pieces};
use crate::{set_bit, chessboard::bitboard::{Bitboard}};

impl Chessboard {
    pub fn square_attacked(&self,attack_maps:&AttackMasks,color:SIDES,square:u8) -> bool {
        /*
        * 0 => occupencies for WHITE 
        * 1 => occupencies for BLACK
        * 2 => occupencies for both*/
        match color {
            SIDES::WHITE => {
                if attack_maps.pawn_attack_masks[1][square as usize] & self.bitboards[Pieces::P] != 0 {
                    return true;
                }
                if attack_maps.king_attack_masks[square as usize] & self.bitboards[Pieces::K] != 0 {
                    return true;
                }
                if attack_maps.knight_attack_masks[square as usize] & self.bitboards[Pieces::N] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::BISHOP,self.occupencies[2],square) & self.bitboards[Pieces::B] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::ROOK,self.occupencies[2],square) & self.bitboards[Pieces::R] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::QUEEN,self.occupencies[2],square) & self.bitboards[Pieces::Q] != 0 {
                    return true;
                }
            }
            SIDES::BLACK => {
                if attack_maps.pawn_attack_masks[0][square as usize] & self.bitboards[Pieces::p] != 0 {
                    return true;
                }
                if attack_maps.king_attack_masks[square as usize] & self.bitboards[Pieces::k] != 0 {
                    return true;
                }
                if attack_maps.knight_attack_masks[square as usize] & self.bitboards[Pieces::n] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::BISHOP,self.occupencies[2],square) & self.bitboards[Pieces::b] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::ROOK,self.occupencies[2],square) & self.bitboards[Pieces::r] != 0 {
                    return true;
                }
                if attack_maps.lookup_slider(SLIDER::QUEEN,self.occupencies[2],square) & self.bitboards[Pieces::q] != 0 {
                    return true;
                }
            }
        }
        return false;
    }
}

// This is a helper that given a chessboard it returns a bitboard of the attacked squares
pub fn attacked_squares(board:&Chessboard,attack_map:&AttackMasks,color:SIDES) -> Bitboard {
    let mut attacked:Bitboard = 0;
    for i in 0..64 {
        if board.square_attacked(&attack_map,color.clone(),i) == true {
            set_bit!(attacked,i);
        }
    }
    attacked
}
