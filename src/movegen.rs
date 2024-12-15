pub mod init;
pub mod movecode;
pub mod movelist;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::{MoveList};
// Am not sure about the below code , but F it ill do this if i get headache ill fix the damn thing 
// Petition : maybe ill include a Vec in the MoveGenerator to store the moves
pub struct MoveGenerator<'a> {
    board:&'a Chessboard,
    attacks:&'a AttackMasks,
    pub moves:MoveList,
}
impl<'a> MoveGenerator<'a> {
    // Creates a new instance depending on a Chessboard and AttackMasks objects , idk if that a
    // good idea or not but hey
    pub fn new(chess:&'a Chessboard,tables:&'a AttackMasks) -> Self {
        Self {
            board:chess,
            attacks:tables,
            moves:MoveList::new(),
        } 
    }
    // We wont change Self here no matter what , this only and only gets us the moves
    pub fn generate_moves(&mut self) {
        self.generate_pawn_moves();
        self.generate_castle_moves();
        self.generate_king_moves();
        self.generate_knight_moves();
        self.generate_bishop_moves();
        self.generate_rook_moves();
        self.generate_queen_moves();
    }
}
