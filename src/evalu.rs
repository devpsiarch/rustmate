/*
* Well define here methodes to evaluate a chessboard*/
mod defs;
use crate::evalu::defs::{PIECES_COST,PIECES_LOCATION_COST,square_mirror};

use crate::chessboard::Chessboard;
use crate::chessboard::defs::{Pieces};
use crate::chessboard::bitboard::{get_lsb};
use crate::{
    pop_bit,
    get_bit,
};
// This function takes a 'clone' of a board once and then usese it up 
pub fn evaluate(mut board:Chessboard) -> i32 {
    let mut value:i32 = 0;
    for i in Pieces::P..=Pieces::k {
        // As long as there is pieces on the board
        let mut lsb:u8; 
        while board.bitboards[i] != 0 {
            lsb = get_lsb(board.bitboards[i]);
            value += PIECES_COST[i];
            // That is a white pieces
            if i >= Pieces::P && i <= Pieces::K {
                value += PIECES_LOCATION_COST[i][lsb as usize];
            }
            // Its black then
            else{
                // i use the i-6 to shift to the white piece to index the 2D arr
                value -= PIECES_LOCATION_COST[i-6][square_mirror(lsb as u8)];
            }
            pop_bit!(board.bitboards[i],lsb);
        }
    }
    value
}
