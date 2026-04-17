pub mod init;
pub mod movecode;
pub mod movelist;
pub mod perft;
use crate::{Chessboard,MoveMask};
use crate::attacks::AttackMasks;
use crate::{MoveList};
use crate::defs::ChessPiece;
// Am not sure about the below code , but F it ill do this if i get headache ill fix the damn thing 
// Petition : maybe ill include a Vec in the MoveGenerator to store the moves
use crate::chessboard::defs::{SQUARE_NAME,UNICODE_PIECES,Pieces};
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

use crate::chessboard::defs::{SQUARE,SIDES,CASTLING_RIGHTS_UPDATE,COLOR};
use crate::Move;

use crate::evalu::defs::PIECES_COST;

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


pub struct UndoMovePacket {
    captured_piece: Option<ChessPiece>,
    castling_rights : u8,
    enpassant_square: u8,
    half_move_clock : u8,
    move_count : u16,
}
impl UndoMovePacket {
    #[inline(always)]
    pub fn new(
        captured_piece: Option<ChessPiece>,
        castling_rights: u8,
        enpassant_square: u8,
        half_move_clock : u8,
        move_count : u16
        ) -> Self {
        Self {
            captured_piece,
            enpassant_square,
            castling_rights,
            half_move_clock,
            move_count
        }
    }
}

// these are the only errors that can occure during making a move
pub enum MakeMoveError {
    Illegal,CaptureConflict
}

// defines the errors that we might fall into if we try to unmake move
pub enum UnmakeMoveError {
    Generic,CaptureMismatch
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
        // Untile now i rememeber that i might wanna reset the array of moves ... what a dummy
        self.moves = MoveList::new(); 
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
    pub fn make_move(&mut self,mv:Move,flag:move_type) -> Result<UndoMovePacket,MakeMoveError> {
        match flag {
            // Making the move normally
            move_type::ALL_MOVES => {
                
                // we create an information packet
                let mut packet = UndoMovePacket::new(
                    None,self.board.castling_rights,self.board.en_passant,self.board.half_move_clock,self.board.move_count
                );

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
                
                // stores the piece that was killed in this move (if it exists)
                let mut killed_in_action: Option<ChessPiece> = None;
                
                // we save the previous ens passant square since its needed for reverse
                let saved_en_passant: u8 = self.board.en_passant;

                let saved_casting_rights = self.board.castling_rights;
                let saved_half_move_clock = self.board.half_move_clock;
                let saved_move_count = self.board.move_count;


                // Now going though all the cases of the move and making the move accordingly 
                // Checking if the move happens to be a capture
                if capture {
                    // Determine the piece range for the side to move
                    let (start, end) = match self.board.side_to_move {
                        SIDES::WHITE => (Pieces::p, Pieces::k),
                        SIDES::BLACK => (Pieces::P, Pieces::K),
                    };

                    // Search for the captured piece and remove it
                    for piece in start..=end {
                        if get_bit!(self.board.bitboards[piece], dst) != 0 {
                            pop_bit!(self.board.bitboards[piece], dst);
                            killed_in_action = Some(piece);
                            packet.captured_piece = Some(piece);
                            break; // Only one piece can occupy a square
                        }
                    }
                }
                // Checking if the piece got promoted 
                if promo != Pieces::NONE as usize {
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
                // if no kings are on the board the program panics , either hanlde it here 
                // or always have kings chilling on the board
                
                if self.square_attacked(self.board.side_to_move.clone(),get_lsb(self.board.bitboards[king]) as u8) == true {
                    // we create a packet from the current board configuration            
                    
                    match self.unmake_move(mv,packet) {
                        Ok(()) => (),
                        Err(_) => panic!("failed to unmake move while the move is illegal."),
                    }

                    // The move is not legal then f this and restore the previous board
                    // What this means is that the move is not made if its not legal
                    return Err(MakeMoveError::Illegal);
                }
                else {
                    // if the move is legal , the move is made and board is updated
                    return Ok(UndoMovePacket::new(killed_in_action,saved_casting_rights,saved_en_passant,saved_half_move_clock,saved_move_count));
                }    
            }
            // Its is said that we do this to avoid the "Horizon effect" , idk why this would help
            // but hey 
            move_type::CAPTURE_MOVE => {
                if get_move_capture!(mv) != 0 {
                    // Thus , it is a capture move
                    return self.make_move(mv,move_type::ALL_MOVES);
                }
                else {
                    return Err(MakeMoveError::CaptureConflict); 
                }
            } 
        }
    }

    pub fn unmake_move(&mut self, mv: Move, information: UndoMovePacket) -> Result<(), UnmakeMoveError> {
            self.board.side_to_move = match self.board.side_to_move {
                SIDES::WHITE => SIDES::BLACK,
                SIDES::BLACK => SIDES::WHITE,
            };

            let us = self.board.side_to_move as usize;
            let them = match self.board.side_to_move {
                SIDES::WHITE => SIDES::BLACK as usize,
                SIDES::BLACK => SIDES::WHITE as usize,
            };

            self.board.en_passant = information.enpassant_square;
            self.board.castling_rights = information.castling_rights;
            self.board.half_move_clock = information.half_move_clock;
            self.board.move_count = information.move_count;

            let src = get_move_src!(mv) as u8;
            let dst = get_move_dst!(mv) as u8;
            let moved_piece = get_move_piece!(mv) as usize;
            let promo = get_move_promotion!(mv) as usize;
            let ep_flag = get_move_enpassant!(mv) != 0;
            let castle_flag = get_move_castle!(mv) != 0;

            if promo != Pieces::NONE as usize {
                pop_bit!(self.board.bitboards[promo], dst);
            } else {
                pop_bit!(self.board.bitboards[moved_piece], dst);
            }
            set_bit!(self.board.bitboards[moved_piece], src);

            pop_bit!(self.board.occupencies[us], dst);
            set_bit!(self.board.occupencies[us], src);

            if let Some(killed) = information.captured_piece {
                if get_move_capture!(mv) == 0 {
                    return Err(UnmakeMoveError::CaptureMismatch);
                }

                let killed_usize = killed as usize;

                if ep_flag {
                    let cap_sq = match self.board.side_to_move {
                        SIDES::WHITE => dst - 8, 
                        SIDES::BLACK => dst + 8,
                    };
                    set_bit!(self.board.bitboards[killed_usize], cap_sq);
                    set_bit!(self.board.occupencies[them], cap_sq);
                } else {
                    set_bit!(self.board.bitboards[killed_usize], dst);
                    set_bit!(self.board.occupencies[them], dst);
                }
            }

            if castle_flag {
                match dst {
                    SQUARE::g1 => {
                        pop_bit!(self.board.bitboards[Pieces::R], SQUARE::f1); 
                        set_bit!(self.board.bitboards[Pieces::R], SQUARE::h1); 
                        pop_bit!(self.board.occupencies[us], SQUARE::f1); // Rook occupancy
                        set_bit!(self.board.occupencies[us], SQUARE::h1);
                    }
                    SQUARE::c1 => {
                        pop_bit!(self.board.bitboards[Pieces::R], SQUARE::d1); 
                        set_bit!(self.board.bitboards[Pieces::R], SQUARE::a1); 
                        pop_bit!(self.board.occupencies[us], SQUARE::d1); // Rook occupancy
                        set_bit!(self.board.occupencies[us], SQUARE::a1);
                    }
                    SQUARE::g8 => {
                        pop_bit!(self.board.bitboards[Pieces::r], SQUARE::f8); 
                        set_bit!(self.board.bitboards[Pieces::r], SQUARE::h8); 
                        pop_bit!(self.board.occupencies[us], SQUARE::f8); // Rook occupancy
                        set_bit!(self.board.occupencies[us], SQUARE::h8);
                    }
                    SQUARE::c8 => {
                        pop_bit!(self.board.bitboards[Pieces::r], SQUARE::d8); 
                        set_bit!(self.board.bitboards[Pieces::r], SQUARE::a8);
                        pop_bit!(self.board.occupencies[us], SQUARE::d8); // Rook occupancy
                        set_bit!(self.board.occupencies[us], SQUARE::a8); 
                    }
                    _ => panic!("What the hell is this castle move ? check generate moves (paniced during unmake move)"),
                }
            }

            self.board.occupencies[COLOR::BOTH] = self.board.occupencies[COLOR::w] | self.board.occupencies[COLOR::b];

            Ok(())
        }

    // the functions below assume that the moves already has been generated
    #[allow(dead_code)] 
    pub fn check_mate(&self) -> bool {
        // this assumes only one king on a board (only on accual games)
        let (king,enemy) = match self.board.side_to_move { 
            SIDES::WHITE => (
                get_lsb(self.board.bitboards[Pieces::K]),
                SIDES::BLACK,
            ),
            SIDES::BLACK => (
                get_lsb(self.board.bitboards[Pieces::k]),
                SIDES::WHITE,
            ),
        };
        if self.square_attacked(enemy,king) == true && self.moves.list.is_empty() {
            true
        }
        else {
            false
        }
    }
    // shut up ... this way it looks pretty ;)
    pub fn stale_mate(&self) -> bool {
        return if self.moves.count == 0 {true} else {false};
    }
    // Pretty prints the moves , this is only for me to not go absolutly insane while trouble
    // shooting
    #[allow(dead_code)]
    pub fn print_all_moves(&self) {
        println!(
        "{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}{:<12}{:<12}",
        "move", "piece", "promoted", "capture", "double", "enpass", "castling","eval"
        );
        for i in 0..self.moves.count {
            let promoted = if get_move_promotion!(self.moves.list[i]) == Pieces::NONE {"NONE"} 
            else {UNICODE_PIECES[get_move_promotion!(self.moves.list[i]) as usize]};
           
            println!("{:<4}{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}{:<12}{:<12}",
                i
                ,SQUARE_NAME[get_move_src!(self.moves.list[i]) as usize].to_owned()+SQUARE_NAME[get_move_dst!(self.moves.list[i]) as usize]
                ,UNICODE_PIECES[get_move_piece!(self.moves.list[i]) as usize]
                ,promoted
                ,get_move_capture!(self.moves.list[i])
                ,get_move_doublejump!(self.moves.list[i])
                ,get_move_enpassant!(self.moves.list[i])
                ,get_move_castle!(self.moves.list[i])
                ,self.evaluate_move(self.moves.list[i]));
        }
        println!("\n\t\t\tTotal moves : {}",self.moves.count);
    }
    pub fn evaluate_move(&self,_some_move:Move) -> f64 {
        // for now evaluate based on MVV-LVA
        if get_move_capture!(_some_move) != 0{
            let mut victim_value = 0.0;
            let mut agressor_value = 0.0;
            let dst = get_move_dst!(_some_move);
            let src = get_move_src!(_some_move);

            for i in Pieces::P..=Pieces::k {
                // match the bitboards with there dst 
                // if they match get the evalution from the table of pices costs
                if self.board.bitboards[i] & (1 << dst) != 0 {
                    victim_value = PIECES_COST[i];
                }
                if self.board.bitboards[i] & ( 1 << src) != 0{
                    agressor_value = PIECES_COST[i];
                }
                if agressor_value != 0.0 && victim_value != 0.0{
                    break;
                }
            }
            return (victim_value * 10.0 - agressor_value).abs();
        }else{
            return 0.0;
        }
    }

    // a function that sorts the moves that have been generated
    pub fn move_order(&mut self){
        let mut stolen_list = std::mem::take(&mut self.moves.list);
        
        stolen_list.sort_by(|a,b| self.evaluate_move(*b).total_cmp(&self.evaluate_move(*a)));
        
        self.moves.list = stolen_list;
    }
}
