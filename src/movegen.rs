pub mod init;
pub mod movecode;
pub mod movelist;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::{MoveList};
// Am not sure about the below code , but F it ill do this if i get headache ill fix the damn thing 
// Petition : maybe ill include a Vec in the MoveGenerator to store the moves

use crate::{
    get_move_dst,
    get_move_src,
    get_move_capture,
    get_move_castle,
    get_move_piece,
    get_move_promotion,
    get_move_doublejump,
    get_move_enpassant,
};
use crate::MoveMask;
use crate::chessboard::defs::{SQUARE,Pieces,SIDES,CASTLING_RIGHTS_UPDATE};
use crate::Move;

use crate::chessboard::bitboard::{get_lsb};
// In the make move function we have to handle the "Horizon effect" althou i am not very familliar
// with said effect but hey
pub enum move_type {
    ALL_MOVES,
    CAPTURE_MOVE,
}


pub struct MoveGenerator<'a> {
    pub board:&'a mut Chessboard,
    attacks:&'a AttackMasks,
    pub moves:MoveList,
}
impl<'a> MoveGenerator<'a> {
    // Creates a new instance depending on a Chessboard and AttackMasks objects , idk if that a
    // good idea or not but hey
    pub fn new(chess:&'a mut Chessboard,tables:&'a AttackMasks) -> Self {
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
    // This is it , the function that makes the moves
    // Also this function is responsible for cheching and setting up game states such as : 
    /*
    * detecting Enpassant
    * Regulating Casle rights
    * Regulating side to move
    */
    /*
    * This is a peudo legal move generator to so we have to check if the move is illegal before
    * returning */
    pub fn make_move(&mut self,mv:Move,flag:move_type) -> bool {
        match flag {
            // Making the move normally
            move_type::ALL_MOVES => {
                // Parse the move first
                let src = get_move_src!(mv) as u8;
                let dst = get_move_dst!(mv) as u8;
                let piece = get_move_piece!(mv) as usize;
                let promo = get_move_promotion!(mv) as usize;
                let capture = if get_move_capture!(mv) != 0 {true} else {false};
                let enpassant = if get_move_enpassant!(mv) != 0 {true} else {false};
                let double = if get_move_doublejump!(mv) != 0 {true} else {false};
                let castle = if get_move_castle!(mv) != 0 {true} else {false};
                
                // Now going though all the cases of the move and making the move accordingly 
                
                // Checking if the move happens to be a capture
                if capture == true {
                    self.board.pop_square(dst);
                    self.board.pop_square(src);
                    // Maybe it is also a Promotion so we check for that too
                    if promo != Pieces::NONE as usize {
                        self.board.spawn_piece(promo,dst);
                    }
                    // Checking for enpassant since it is considered as a capture too
                    if enpassant == true {
                        self.board.spawn_piece(piece,dst);
                        // Checking for each color 
                        match self.board.side_to_move {
                            SIDES::WHITE => self.board.pop_square(dst+8),
                            SIDES::BLACK => self.board.pop_square(dst-8),
                        }
                    }
                    // or just a normal capture
                    else {
                        self.board.spawn_piece(piece,dst);
                    }
                }
                // We reset the enpassant square each move is made because , in chess rule book,
                // enpassant is availble in one turn only , unless taken
                self.board.en_passant = SQUARE::NO_SQUARE as u8;
                // Making the castle move
                if castle == true {
                    self.board.pop_square(src);
                    self.board.spawn_piece(piece,dst);

                    // Going though the cases of where would the caslte be after the move
                    match dst {
                        // I didnt wanna split them even more cuz i think this is fine for now
                        // The first two casers are for white castles 
                        SQUARE::g1 => {
                            self.board.pop_square(SQUARE::h1);
                            self.board.spawn_piece(Pieces::R,SQUARE::f1);
                        }
                        SQUARE::c1 => {
                            self.board.pop_square(SQUARE::a1);
                            self.board.spawn_piece(Pieces::R,SQUARE::d1);
                        }
                        // And these are for black casltes 
                        SQUARE::g8 => {
                            self.board.pop_square(SQUARE::h8);
                            self.board.spawn_piece(Pieces::r,SQUARE::f8);
                        }
                        SQUARE::c8 => {
                            self.board.pop_square(SQUARE::a8);
                            self.board.spawn_piece(Pieces::r,SQUARE::d8);
                        }
                        // If we panic here that means there is an error in the move generation
                        // methode go check it out
                        _ => panic!(),
                    }
                }
                // Making the double move and updating the en_passant square since we already reset
                // it before hand after we made the enpassant move if it was availble
                if double == true {
                    self.board.pop_square(src);
                    self.board.spawn_piece(piece,dst);
                    match self.board.side_to_move {
                        SIDES::WHITE => self.board.en_passant = dst + 8,
                        SIDES::BLACK => self.board.en_passant = dst - 8,
                    }
                }
                else {
                    // A normal move happened nothing crazy
                    self.board.pop_square(src);
                    // checking if this nomal move happens to be a Promotion
                    if promo != Pieces::NONE as usize {
                        self.board.spawn_piece(promo,dst);
                    }
                    // else we just move
                    else {
                        self.board.spawn_piece(piece,dst);
                    }
                }
                // updating the caslte rights after all pieces have moved , if you dont get it ,
                // check the caslte update array in defs 
                self.board.castling_rights &= CASTLING_RIGHTS_UPDATE[src as usize];
                self.board.castling_rights &= CASTLING_RIGHTS_UPDATE[dst as usize];
                // Changing the side to move 
                match self.board.side_to_move {
                    SIDES::WHITE => self.board.side_to_move = SIDES::BLACK,
                    SIDES::BLACK => self.board.side_to_move = SIDES::WHITE,
                }
                // Cheking if said move makes the king in check
                match self.board.side_to_move {
                    SIDES::WHITE => {
                        if self.square_attacked(SIDES::WHITE,get_lsb(self.board.bitboards[Pieces::k]) as u8) == true {
                            return false;
                        }
                    }
                    SIDES::BLACK => {
                        if self.square_attacked(SIDES::BLACK,get_lsb(self.board.bitboards[Pieces::K]) as u8) == true {
                            return false;
                        }
                    }
                } 
                return true; 
            }
            // Its is said that we do this to avoid the "Horizon effect" , idk why this would help
            // but hey 
            move_type::CAPTURE_MOVE => {
                if get_move_capture!(mv) == 1 {
                    return self.make_move(mv,move_type::ALL_MOVES);
                }
                return true; 
            } 
        }
    } 
}
