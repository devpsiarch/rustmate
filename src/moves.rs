pub mod init;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::bitboard::{Bitboard};
use crate::defs::Pieces;

// Am not sure about the below code , but F it ill do this if i get headache ill fix the damn thing 
// Petition : maybe ill include a Vec in the MoveGenerator to store the moves
pub struct MoveGenerator<'a> {
    board:&'a Chessboard,
    attacks:&'a AttackMasks,
}
impl<'a> MoveGenerator<'a> {
    // Creates a new instance depending on a Chessboard and AttackMasks objects , idk if that a
    // good idea or not but hey
    pub fn new(chess:&'a Chessboard,tables:&'a AttackMasks) -> Self {
        Self {
            board:chess,
            attacks:tables,
        } 
    }
    // We wont change Self here no matter what , this only and only gets us the moves
    pub fn Generate_moves(&self) {
        let mut src_square:u8;
        let mut dst_square:u8;

        let mut bitboard:Bitboard;
        let mut attack_tables:Bitboard;

        for piece in Pieces::P..=Pieces::k {
            bitboard = self.board.bitboards[piece];
            // Getting moves for each piece here
        }
    }
}
