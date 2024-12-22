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
use crate::chessboard::defs::{SQUARE,Pieces};
use crate::Move;
// This list am gonna use to errors while parsiong the move parts in the MakeMove methode down
// below
const MOVE_PARTS: [&str;8] = [
    "Source square","Destination square","Piece moved","Promotion status"
    ,"Capture status","Double jump status","Enpassant status","Castling status"
];
// And this is the return type of said methode 
type MakeMoveResult = Result<(),u8>;
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
    * Setting up and checking if an Enpassant is availabe after a move
    * Regulating the Casle rights after a moves i made and after a casle was made
    */ // These will be implmented later for now , making the pieces move is enough
    pub fn make_move(&mut self,mv:Move,flag:move_type) -> MakeMoveResult {
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
                    // or just a normal capture
                    else {
                        self.board.spawn_piece(piece,dst);
                    }
                    return Ok(());
                }
                // Checking for enpassant
                if enpassant == true {
                    self.board.spawn_piece(piece,dst);
                    // Check for each color 
                    // white here 
                    if piece <= Pieces::P {
                        self.board.pop_square(dst+8);
                    }
                    // black here 
                    else {
                        self.board.pop_square(dst-8);
                    }
                    return Ok(());
                }
                // Making the double move and handling arangements
                if double == true {
                    self.board.pop_square(src);
                    self.board.spawn_piece(piece,dst);
                    return Ok(());
                }
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
                            self.board.spawn_piece(Pieces::R,SQUARE::f8);
                        }
                        SQUARE::c8 => {
                            self.board.pop_square(SQUARE::a8);
                            self.board.spawn_piece(Pieces::R,SQUARE::d8);
                        }
                        // Else if the cases arent matched then something happends and we retuns 
                        // a error status 7
                        _ => return Err(7),
                    }
                    return Ok(());
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
                    return Ok(());
                }
                Ok(())
            }
            // Its is said that we do this to avoid the "Horizon effect" , idk why this would help
            // but hey 
            move_type::CAPTURE_MOVE => {
                if get_move_capture!(mv) == 1 {
                    return self.make_move(mv,move_type::ALL_MOVES);
                }
                Ok(())
            } 
        }
    } 
}
