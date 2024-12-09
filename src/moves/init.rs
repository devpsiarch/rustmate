/*
* Well define some essential methodes here*/
use crate::moves::MoveGenerator; 
use crate::chessboard::defs::{SQUARE_NAME,SIDES,SLIDER,Pieces,SQUARE};
use crate::{kill_board,get_bit,pop_bit,set_bit, chessboard::bitboard::{Bitboard,get_lsb}};

// These are used to check if a pawn can jump two squares for black and white 
const RANK_7:Bitboard = 65280;
const RANK_2:Bitboard = 71776119061217280;
// These are used to detect promotions
const RANK_1:Bitboard = 18374686479671623680;
const RANK_8:Bitboard = 255;

impl<'a> MoveGenerator<'_> {
    pub fn square_attacked(&self,color:SIDES,square:u8) -> bool {
        /*
        * 0 => board.occupencies for WHITE 
        * 1 => board.occupencies for BLACK
        * 2 => board.occupencies for both*/
        match color {
            SIDES::WHITE => {
                if self.attacks.pawn_attack_masks[1][square as usize] & self.board.bitboards[Pieces::P] != 0 {
                    return true;
                }
                if self.attacks.king_attack_masks[square as usize] & self.board.bitboards[Pieces::K] != 0 {
                    return true;
                }
                if self.attacks.knight_attack_masks[square as usize] & self.board.bitboards[Pieces::N] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::B] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::R] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::Q] != 0 {
                    return true;
                }
            }
            SIDES::BLACK => {
                if self.attacks.pawn_attack_masks[0][square as usize] & self.board.bitboards[Pieces::p] != 0 {
                    return true;
                }
                if self.attacks.king_attack_masks[square as usize] & self.board.bitboards[Pieces::k] != 0 {
                    return true;
                }
                if self.attacks.knight_attack_masks[square as usize] & self.board.bitboards[Pieces::n] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::b] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::r] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[2],square) 
                                    & self.board.bitboards[Pieces::q] != 0 {
                    return true;
                }
            }
        }
        return false;
    }
    // This is a helper that given a chessboard it returns a bitboard of the attacked squares
    pub fn attacked_squares(&self,color:SIDES) -> Bitboard {
        let mut attacked:Bitboard = 0;
        for i in 0..64 {
            if self.square_attacked(color.clone(),i) == true {
                set_bit!(attacked,i);
            }
        }
        attacked
    }
    // Here are the methodes to generate moves for each piece
    // i dont know yet of we will store the moves in the object or not but for now ill just print
    // them
    pub fn generate_pawn_moves(&self,color:SIDES) {
        let mut bitboard:Bitboard = 0;
        let mut helper_bitboard:Bitboard = 0;       // This will come in handy
        match color {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::P];
                let mut src:usize;
                let mut dst:usize;
                while bitboard != 0 {
                    src = get_lsb(bitboard) as usize;
                    dst = src-8;
                    // Here we use the helper to see if we are in a rank that lets us double jump
                    set_bit!(helper_bitboard,dst);
                    // Check if we can go forward by ANDING the both occupencies for BLACK and WHITE
                    if helper_bitboard & self.board.occupencies[2] == 0 {
                        // Check if moving forward is a promotion
                        if helper_bitboard & RANK_1 != 0 || helper_bitboard & RANK_8 != 0{
                            println!("Pawn promotes from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst]);
                        }
                        // Then the move is just a move forward
                        else{
                            println!("Pawn move from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst]);
                        }
                    }
                    // Check if we can double jump
                    kill_board!(helper_bitboard);
                    set_bit!(helper_bitboard,src);
                    if helper_bitboard & RANK_2 != 0 {
                        kill_board!(helper_bitboard);
                        set_bit!(helper_bitboard,dst-8);
                        if helper_bitboard & self.board.occupencies[2] == 0 {
                            println!("Pawn doubles jumps from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst-8]);
                        }
                    }
                    // Double jump if piece is in its double jump rank 2 for WHITE and 7 for BLACK 
                    kill_board!(helper_bitboard);
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::p];
            }
        }


    }  
}
