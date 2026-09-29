/*
* Well define here methodes to evaluate a chessboard*/
pub mod defs;
use crate::evalu::defs::{PIECES_COST,PIECES_LOCATION_COST_ER,PIECES_LOCATION_COST_MG,PIECES_LOCATION_COST_EG,square_mirror};

use crate::chessboard::Chessboard;
use crate::chessboard::defs::{Pieces};
use crate::chessboard::bitboard::{get_lsb};
use crate::{
    pop_bit,
    get_bit,
};
// This function takes a 'clone' of a board once and then usese it up 
pub fn evaluate(mut board:Chessboard) -> f64 {
    let mut value:f64 = 0.0;
    let mut cost_map: Option<&[[f64; 64]; 6]> = None;
    if board.move_count <= 15 {
        cost_map = Some(&PIECES_LOCATION_COST_ER);
    } else if board.move_count <= 40 {
        cost_map = Some(&PIECES_LOCATION_COST_MG);
    } else {
        cost_map = Some(&PIECES_LOCATION_COST_EG);
    }

    for i in Pieces::P..=Pieces::k {
        // As long as there is pieces on the board
        while board.bitboards[i] != 0 {
            let lsb:u8 = get_lsb(board.bitboards[i]);
            value += PIECES_COST[i];
            // That is a white pieces
            if i >= Pieces::P && i <= Pieces::K {
                value += cost_map.expect("Failed to choose piece map cost!")[i][square_mirror(lsb as u8)];
            }
            // Its black then
            else{
                // i use the i-6 to shift to the white piece to index the 2D arr
                value -= cost_map.expect("Failed to choose piece map cost!")[i-6][lsb as usize];
            }
            pop_bit!(board.bitboards[i],lsb);
        }
    }
    value
}
