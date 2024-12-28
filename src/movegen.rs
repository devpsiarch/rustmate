pub mod init;
pub mod movecode;
pub mod movelist;
pub mod perft;
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
use crate::chessboard::defs::{SQUARE,Pieces,SIDES,CASTLING_RIGHTS_UPDATE,COLOR};
use crate::Move;

use crate::chessboard::bitboard::{get_lsb};
// In the make move function we have to handle the "Horizon effect" althou i am not very familliar
// with said effect but hey
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum move_type {
    ALL_MOVES,
    CAPTURE_MOVE,
}

use crate::get_bit;
use crate::set_bit;
use crate::pop_bit;


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
                // create a clone of the current board 
                // this function will never change the board if the move is not legal
                let copy = self.board.clone();
                // Parse the move first
                let src = get_move_src!(mv) as u8;
                let dst = get_move_dst!(mv) as u8;
                let piece = get_move_piece!(mv) as usize;
                let promo = get_move_promotion!(mv) as usize;
                let capture = if get_move_capture!(mv) != 0 {true} else {false};
                let enpassant = if get_move_enpassant!(mv) != 0 {true} else {false};
                let double = if get_move_doublejump!(mv) != 0 {true} else {false};
                let castle = if get_move_castle!(mv) != 0 {true} else {false};
               
                // Making the move first disregarding the occupancies they will be handled later 
                pop_bit!(self.board.bitboards[piece],src);   
                set_bit!(self.board.bitboards[piece],dst);   
                
                // Now going though all the cases of the move and making the move accordingly 
                // Checking if the move happens to be a capture
                if capture == true {
                    let start:usize;
                    let end:usize;
                    // Setting up which side are we capturing 
                    match self.board.side_to_move {
                        SIDES::WHITE => {
                            start = Pieces::p;
                            end = Pieces::k;
                        }
                        SIDES::BLACK => {
                            start = Pieces::P;
                            end = Pieces::K;
                        }
                    }
                    // searching who got capture and poping him from the board
                    for i in start..=end {
                        if get_bit!(self.board.bitboards[i],dst) != 0 {
                            pop_bit!(self.board.bitboards[i],dst);
                            break; 
                            // we break here because we assumse only one piece can be in a square
                            // at time
                        }
                    }
                }
                // Checking if the piece got promoted 
                if promo != Pieces::NONE as usize{
                    // pop the pawn that got promoted
                    match self.board.side_to_move {
                        SIDES::WHITE => pop_bit!(self.board.bitboards[Pieces::P],dst),
                        SIDES::BLACK => pop_bit!(self.board.bitboards[Pieces::p],dst),
                    }
                    // replace it with the promoted piece
                    set_bit!(self.board.bitboards[promo],dst);
                }
                // Checking if the move is enpassant
                if enpassant == true {
                     match self.board.side_to_move {
                        SIDES::WHITE => pop_bit!(self.board.bitboards[Pieces::p],dst+8),
                        SIDES::BLACK => pop_bit!(self.board.bitboards[Pieces::P],dst-8),
                    }                   
                } 
                // Resetting the enpassant square from the board
                self.board.en_passant = SQUARE::NO_SQUARE;
                // Handelling the double jump pawn , 
                // and also updating the enpasasnt cue these are wher they become availble
                if double == true {
                     match self.board.side_to_move {
                        SIDES::WHITE => self.board.en_passant = dst + 8,
                        SIDES::BLACK => self.board.en_passant = dst - 8,
                    }
                }
                // Making the castle move
                if castle == true {
                    // Going though the cases of where would the caslte be after the move
                    match dst {
                        // I didnt wanna split them even more cuz i think this is fine for now
                        // The first two casers are for white castles 
                        SQUARE::g1 => {
                            pop_bit!(self.board.bitboards[Pieces::R],SQUARE::h1); 
                            set_bit!(self.board.bitboards[Pieces::R],SQUARE::f1); 
                        }
                        SQUARE::c1 => {
                            pop_bit!(self.board.bitboards[Pieces::R],SQUARE::a1); 
                            set_bit!(self.board.bitboards[Pieces::R],SQUARE::d1); 
                        }
                        // And these are for black casltes 
                        SQUARE::g8 => {
                            pop_bit!(self.board.bitboards[Pieces::r],SQUARE::h8); 
                            set_bit!(self.board.bitboards[Pieces::r],SQUARE::f8); 
                        }
                        SQUARE::c8 => {
                            pop_bit!(self.board.bitboards[Pieces::r],SQUARE::a8); 
                            set_bit!(self.board.bitboards[Pieces::r],SQUARE::d8); 
                        }
                        // If we panic here that means there is an error in the move generation
                        // methode go check it out
                        _ => panic!("What the hell is this caslte move ? check generate moves"),
                    }
                }
                // updating the caslte rights after all pieces have moved , if you dont get it ,
                // check the caslte update array in defs 
                self.board.castling_rights &= CASTLING_RIGHTS_UPDATE[src as usize];
                self.board.castling_rights &= CASTLING_RIGHTS_UPDATE[dst as usize];
                
                // Updating the occupencies here
                // First we reset no less headache
                self.board.reset_occupencies();
                // Populating 
                for i in Pieces::P..=Pieces::K {
                    self.board.occupencies[COLOR::w] |= self.board.bitboards[i];
                }
                for i in Pieces::p..=Pieces::k {
                    self.board.occupencies[COLOR::b] |= self.board.bitboards[i];
                }
                self.board.occupencies[COLOR::BOTH] |= self.board.occupencies[COLOR::b];
                self.board.occupencies[COLOR::BOTH] |= self.board.occupencies[COLOR::w];
                // Changing the side to move 
                match self.board.side_to_move {
                    SIDES::WHITE => self.board.side_to_move = SIDES::BLACK,
                    SIDES::BLACK => self.board.side_to_move = SIDES::WHITE,
                }
                // Cheking if said move makes the king in check
                let king:usize;
                match self.board.side_to_move {
                    SIDES::WHITE => {
                        king = Pieces::k; 
                    }
                    SIDES::BLACK => {
                        king = Pieces::K; 
                    }
                }
                if self.square_attacked(self.board.side_to_move.clone(),get_lsb(self.board.bitboards[king]) as u8) == true {
                    // The move is not legal then f this and restore the previous board
                    // What this means is that the move is not made if its not legal
                    self.board.restore_board(copy);
                    return false;
                }
                else {
                    return true; 
                }    
            }
            // Its is said that we do this to avoid the "Horizon effect" , idk why this would help
            // but hey 
            move_type::CAPTURE_MOVE => {
                if get_move_capture!(mv) == 1 {
                    return self.make_move(mv,move_type::ALL_MOVES);
                }
                else {
                    return false; 
                }
            } 
        }
    } 
}
