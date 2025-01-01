/*
* Well define some essential methodes here and while were at maybe ill just impliment a little 
* Pseudo move genearator nothing serious ... that was sarcasm BTW 
* NOTE: IDK if declaring a small varible to store src and dst for each eteration is a good idea , i
* dont know if that will cost more time in the future , so i gotta do tests and check if making
* declaring thme once and changing them makes a difference from just making and freeing them when
* out scop
*/
/*
* I have commented out the print statement just in case i broke somthing here and am later obliged
* to come here again in the generate move function and fix it*/
/*
* Also , please remove the printing of moves alr ? it takes time and its ugly , maybe comment them
* out because you are such a dummy ull prolly forget what the code do if you ran into trouble , idk
* just stop prining
* */
use crate::movegen::MoveGenerator; 
use crate::chessboard::defs::{COLOR,SIDES,SLIDER,SQUARE,Castle,Pieces};
use crate::{get_bit,pop_bit,set_bit, chessboard::bitboard::{Bitboard,get_lsb}};
use crate::{
    encode_move,
};
/*
* For now the move genearation prints the available moves but later on , either we return a vec of
* the moves or store them in a attribute of said object , and thats only when we encoded them in
* some way or another*/
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
                if self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[COLOR::BOTH],square) 
                                    & self.board.bitboards[Pieces::B] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[COLOR::BOTH],square) 
                                    & self.board.bitboards[Pieces::R] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[COLOR::BOTH],square) 
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
                if self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[COLOR::BOTH],square) 
                                    & self.board.bitboards[Pieces::b] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[COLOR::BOTH],square) 
                                    & self.board.bitboards[Pieces::r] != 0 {
                    return true;
                }
                if self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[COLOR::BOTH],square) 
                                    & self.board.bitboards[Pieces::q] != 0 {
                    return true;
                }
            }
        }
        return false;
    }
    // This is a helper that given a chessboard it returns a bitboard of the attacked squares
    // Wont be used at all in the move generation , mearly a tool to help me check if code works
    // fine 
    #[allow(dead_code)]
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
    pub fn generate_pawn_moves(&mut self) {
        let mut bitboard:Bitboard;
        let mut atk:Bitboard;       // This will come in handy
        // Change it to SIDE TO MOVE 
        match self.board.side_to_move {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::P];
                if bitboard == 0 {
                    return;
                }
                let mut src:i8;
                let mut dst:i8;
                while bitboard != 0 {
                    src = get_lsb(bitboard) as i8;
                    dst = src-8;
                    // Check if we can generate "quite" moves as in jumping forward
                    if dst >= SQUARE::a8.try_into().unwrap() && get_bit!(self.board.occupencies[COLOR::BOTH],dst) == 0 {
                        // Check if the newt jump ahead is a pawn promotion which happens if the
                        // dst is in the last rank 
                        if src >= SQUARE::a7.try_into().unwrap() && src <=SQUARE::h7.try_into().unwrap() {
                            // Well consider every possible promotions here
                            //Here am generating the moves and storing them in the moves under self
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::Q as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::R as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::B as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::N as u32,0,0,0,0));
                            // AM only printing the moves to see if genrating the moves is going good
                            //println!("Pawn from {} to {} Promotion to queen ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("Pawn from {} to {} Promotion to rook ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("Pawn from {} to {} Promotion to bishop ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("Pawn from {} to {} Promotion to knight ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        else {
                            // We check the normal fashion 1 square ahead move
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::NONE,0,0,0,0));
                            //println!("Pawn from {} to {} ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            // We check if we can double jump here only if we are in the starting rank
                            if src >= SQUARE::a2.try_into().unwrap() && src <= SQUARE::h2.try_into().unwrap() 
                                && get_bit!(self.board.occupencies[COLOR::BOTH],dst-8) == 0{
                                self.moves.add_move(encode_move!(src as u32,(dst-8) as u32,Pieces::P as u32,Pieces::NONE,0,1,0,0));
                                //println!("Pawn from {} to {} double jump ",SQUARE_NAME[src as usize],SQUARE_NAME[(dst-8) as usize]);
                            }
                        }
                    }
                    // Checking if "Attacks" are available
                    atk = self.attacks.pawn_attack_masks[COLOR::w][src as usize] & self.board.occupencies[COLOR::b];   
                    if atk != 0 {
                        while atk != 0 {
                            let killed = get_lsb(atk);
                            // Check if the Capture also is a promotion 
                            if killed >= SQUARE::a8.try_into().unwrap() && killed <=SQUARE::h8.try_into().unwrap() {
                                // Well consider every possible promotions here
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::P as u32,Pieces::Q as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::P as u32,Pieces::R as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::P as u32,Pieces::B as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::P as u32,Pieces::N as u32,1,0,0,0));
                                //println!("Pawn Capture from {} to {} Promotion to queen ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to rook ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to bishop ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to knight ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                            }
                            else {
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::P as u32,Pieces::NONE,1,0,0,0));
                                //println!("Pawn Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                            }
                            pop_bit!(atk,killed);
                        } 
                    }
                    // Finally , We check got enpassant capture by Checking one of the boards
                    // attributes , SO the availability of a enpassant move is not handeled here.
                    if self.board.en_passant != SQUARE::NO_SQUARE {
                        atk = self.attacks.pawn_attack_masks[COLOR::w][src as usize] & (1u64 << self.board.en_passant);
                        // Check if its available
                        if atk != 0 {
                            dst = get_lsb(atk) as i8;
                            //println!("Pawn enpassant from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::P as u32,Pieces::NONE,1,0,1,0));
                        }
                    } 
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::p];
                if bitboard == 0 {
                    return;
                }
                let mut src:i8;
                let mut dst:i8;
                while bitboard != 0 {
                    src = get_lsb(bitboard) as i8;
                    dst = src+8;
                    // Check if we can generate "quite" moves as in jumping forward
                    if dst <= SQUARE::h1.try_into().unwrap() && get_bit!(self.board.occupencies[COLOR::BOTH],dst) == 0 {
                        // Check if the newt jump ahead is a pawn promotion which happens if the
                        // dst is in the last rank 
                        if src >= SQUARE::a2.try_into().unwrap() && src <=SQUARE::h2.try_into().unwrap() {
                            // Well consider every possible promotions here
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::q as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::r as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::b as u32,0,0,0,0));
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::n as u32,0,0,0,0));
                            //println!("pawn from {} to {} Promotion to queen ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("pawn from {} to {} Promotion to rook ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("pawn from {} to {} Promotion to bishop ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            //println!("pawn from {} to {} Promotion to knight ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        else {
                            // We check the normal fashion 1 square ahead move 
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::NONE,0,0,0,0));
                            //println!("pawn from {} to {} ",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                            // We check if we can double jump here only if we are in the starting rank
                            if src >= SQUARE::a7.try_into().unwrap() && src <= SQUARE::h7.try_into().unwrap() 
                                && get_bit!(self.board.occupencies[COLOR::BOTH],dst+8) == 0{
                                self.moves.add_move(encode_move!(src as u32,(dst+8) as u32,Pieces::p as u32,Pieces::NONE,0,1,0,0));
                                //println!("pawn from {} to {} double jump ",SQUARE_NAME[src as usize],SQUARE_NAME[(dst+8) as usize]);
                            }
                        }
                    }
                    // Checking if "Attacks" are available
                    atk = self.attacks.pawn_attack_masks[COLOR::b][src as usize] & self.board.occupencies[COLOR::w];   
                    if atk != 0 {
                        while atk != 0 {
                            let killed = get_lsb(atk);
                            // Check if the Capture also is a promotion 
                            if killed >= SQUARE::a1.try_into().unwrap() && killed <=SQUARE::h1.try_into().unwrap() {
                                // Well consider every possible promotions here
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::p as u32,Pieces::q as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::p as u32,Pieces::r as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::p as u32,Pieces::b as u32,1,0,0,0));
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::p as u32,Pieces::n as u32,1,0,0,0));
                                //println!("Pawn Capture from {} to {} Promotion to queen ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to rook ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to bishop ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                                //println!("Pawn Capture from {} to {} Promotion to knight ",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                            }
                            else {
                                // Check the normal capture
                                self.moves.add_move(encode_move!(src as u32,killed as u32,Pieces::p as u32,Pieces::NONE,1,0,0,0));
                                //println!("Pawn Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[killed as usize]);
                            }
                            pop_bit!(atk,killed);
                        } 
                    }
                    // Finally , We check got enpassant capture by Checking one of the boards
                    // attributes , SO the availability of a enpassant move is not handeled here.
                    if self.board.en_passant != SQUARE::NO_SQUARE {
                        atk = self.attacks.pawn_attack_masks[COLOR::b][src as usize] & (1u64 << self.board.en_passant);
                        // Check if its available
                        if atk != 0 {
                            dst = get_lsb(atk) as i8;
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::p as u32,Pieces::NONE,1,0,1,0));
                            //println!("Pawn enpassant from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                    } 
                    pop_bit!(bitboard,src);
                }
            }               
        }


    }
    // This methodes as the name suggest it generates the castle moves
    pub fn generate_castle_moves(&mut self) {
        // Here out only job is to generate castle moves GIVEN the caslte rights , 
        // In other words , we only check IF WE CAN CASTLE , and no if WE HAVE CASLTE RIGHTS
        // Castle Rights only change if one of the rooks or the king moves from a specific location
        // and that happens when we MAKE A MOVE , so that will be handeled by the MAKING MOVES
        // mothodes not here what so ever 
        match self.board.side_to_move {
            SIDES::WHITE => {
                // We check if the castling rights are set 
                // then check of the "Path is cleared" for making the move
                // then we check if the "Path" is not attacked by the enemy
                if self.board.castling_rights & Castle::K != 0 
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::f1) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::g1) == 0
                && self.square_attacked(SIDES::BLACK,SQUARE::f1) == false              
                && self.square_attacked(SIDES::BLACK,SQUARE::g1) == false //where kings lands 
                && self.square_attacked(SIDES::BLACK,SQUARE::e1) == false{// Check king square
                    self.moves.add_move(encode_move!(SQUARE::e1 as u32,SQUARE::g1 as u32,Pieces::K as u32,Pieces::NONE,0,0,0,1));
                    //println!("white King caslte from {} to {}",SQUARE_NAME[SQUARE::e1 as usize],SQUARE_NAME[SQUARE::g1 as usize]);
                }

                if self.board.castling_rights & Castle::Q != 0 
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::b1) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::c1) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::d1) == 0
                && self.square_attacked(SIDES::BLACK,SQUARE::d1) == false 
                && self.square_attacked(SIDES::BLACK,SQUARE::c1) == false  //King landing
                && self.square_attacked(SIDES::BLACK,SQUARE::e1) == false {// for king square
                    self.moves.add_move(encode_move!(SQUARE::e1 as u32,SQUARE::c1 as u32,Pieces::K as u32,Pieces::NONE,0,0,0,1));
                    //println!("white queen caslte from {} to {}",SQUARE_NAME[SQUARE::e1 as usize],SQUARE_NAME[SQUARE::c1 as usize]);
                }
            }
            SIDES::BLACK => {
                if self.board.castling_rights & Castle::k != 0 
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::f8) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::g8) == 0
                && self.square_attacked(SIDES::WHITE,SQUARE::f8) == false
                && self.square_attacked(SIDES::WHITE,SQUARE::g8) == false //king landing here
                && self.square_attacked(SIDES::WHITE,SQUARE::e8) == false{// Check kings square
                    self.moves.add_move(encode_move!(SQUARE::e8 as u32,SQUARE::g8 as u32,Pieces::k as u32,Pieces::NONE,0,0,0,1));
                    //println!("black king caslte from {} to {}",SQUARE_NAME[SQUARE::e8 as usize],SQUARE_NAME[SQUARE::g8 as usize]);
                }

                if self.board.castling_rights & Castle::q != 0 
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::b8) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::c8) == 0
                && get_bit!(self.board.occupencies[COLOR::BOTH],SQUARE::d8) == 0
                && self.square_attacked(SIDES::WHITE,SQUARE::d8) == false 
                && self.square_attacked(SIDES::WHITE,SQUARE::c8) == false //king landing
                && self.square_attacked(SIDES::WHITE,SQUARE::e8) == false{// king square
                    self.moves.add_move(encode_move!(SQUARE::e8 as u32,SQUARE::c8 as u32,Pieces::k as u32,Pieces::NONE,0,0,0,1));
                    //println!("black queen caslte from {} to {}",SQUARE_NAME[SQUARE::e8 as usize],SQUARE_NAME[SQUARE::c8 as usize]);
                }
            }
        }
    }
    // Here we generate the moves for King piece
    pub fn generate_king_moves(&mut self) {
        // Same buisness , get attack table and deal with each landing square 
        // BUT !!! we have to make sure that the square that we land on doesnt make us in check 
        // SO THEY LANDINGS MUST BE NOT ATTACKED
        let mut atk:Bitboard;
        match self.board.side_to_move {
            SIDES::WHITE => {
                // Here we assume that there exists only one king in the whole board which is true 
                // WARNING : HAVING MORE KINGS FOR EXPIREMENTATION REQUESTS A CHANGE HERE !!!
                if self.board.bitboards[Pieces::K] == 0 {
                    return;
                }
                let src:usize = get_lsb(self.board.bitboards[Pieces::K]) as usize;
                // "I trusted you king ..." just checking for friendly fire
                atk = self.attacks.king_attack_masks[src] & !self.board.occupencies[COLOR::w];
                let mut dst:u8;
                while atk != 0 {
                    dst = get_lsb(atk);
                    // Making sure the landing square is not attacked by the enemy
                    if self.square_attacked(SIDES::BLACK,dst) == false {
                        // If the move is a Capture move
                        if get_bit!(self.board.occupencies[COLOR::b],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::K as u32,Pieces::NONE,1,0,0,0));
                            //println!("White King Captures from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst as usize]);
                        }
                        // Else it is just a normal move
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::K as u32,Pieces::NONE,0,0,0,0));
                            //println!("White King moves from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst as usize]);
                        }
                    }
                    pop_bit!(atk,dst);
                }
            }
            SIDES::BLACK => {
                // Here we assume that there exists only one king in the whole board which is true 
                // WARNING : HAVING MORE KINGS FOR EXPIREMENTATION REQUESTS A CHANGE HERE !!!
                if self.board.bitboards[Pieces::k] == 0 {
                    return;
                }
                let src:usize = get_lsb(self.board.bitboards[Pieces::k]) as usize;
                // Why so much comment you ask ? well cur am an idiot and will forget what each
                // line do , bear with me ... again , i dont want friendly KIA
                atk = self.attacks.king_attack_masks[src] & !self.board.occupencies[COLOR::b];
                if atk == 0 {
                    return ;
                }
                let mut dst:u8;
                while atk != 0 {
                    dst = get_lsb(atk);
                    // Making sure the landing square is not attacked by the enemy
                    if self.square_attacked(SIDES::WHITE,dst) == false {
                        // If the move is a Capture move
                        if get_bit!(self.board.occupencies[COLOR::w],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::k as u32,Pieces::NONE,1,0,0,0));
                            //println!("Black King Captures from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst as usize]);
                        }
                        // Else it is just a normal move
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::k as u32,Pieces::NONE,0,0,0,0));
                            //println!("Black King moves from {} to {}",SQUARE_NAME[src],SQUARE_NAME[dst as usize]);
                        }
                    }
                    pop_bit!(atk,dst);
                }
            }
        }
    }
    // Getting the attacks available for knight depending on who is up for the next to move
    pub fn generate_knight_moves(&mut self) {
        // We get the attack maps and make sure if we land in a friendly piece , then we skip that
        // move and go for another
        let mut bitboard:Bitboard;
        let mut atk:Bitboard;
        match self.board.side_to_move {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::N];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    // We AND it with the NOT of its friendly to make sure it wont capture friendly
                    // pieces
                    atk = self.attacks.knight_attack_masks[src as usize] & !self.board.occupencies[COLOR::w];
                    let mut dst:u8;
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // Here am specifing if am CAPTURING a piece or just chilling
                        // The case for capturing
                        if get_bit!(self.board.occupencies[COLOR::b],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::N as u32,Pieces::NONE,1,0,0,0));
                            //println!("White knight Capture from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // Case for a chill knight ... 
                        else{
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::N as u32,Pieces::NONE,0,0,0,0));
                            //println!("White knight from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::n];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    // Check we are not betraying our own 
                    atk = self.attacks.knight_attack_masks[src as usize] & !self.board.occupencies[COLOR::b];
                    let mut dst:u8;
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // Here am specifing if am CAPTURING a piece or just chilling
                        // The case for capturing
                        if get_bit!(self.board.occupencies[COLOR::w],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::n as u32,Pieces::NONE,1,0,0,0));
                            //println!("Black knight Capture from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // Case for a chill knight ... 
                        else{
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::n as u32,Pieces::NONE,0,0,0,0));
                            //println!("Black knight from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
        }
    }
    // Gets possible attacks from bishops
    // This methode will bassicly be the same for each slider piece
    pub fn generate_bishop_moves(&mut self) {
        let mut bitboard:Bitboard;
        let mut atk:Bitboard;
        match self.board.side_to_move {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::B];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    // Cheking for betrayals .... am going insane
                    atk = self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::w]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::b],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::B as u32,Pieces::NONE,1,0,0,0));
                            //println!("white Bishop Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::B as u32,Pieces::NONE,0,0,0,0));
                            //println!("white Bishop moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::b];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    atk = self.attacks.lookup_slider(SLIDER::BISHOP,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::b]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // "DO NOT TRUST SHAPERD !!" 
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::w],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::b as u32,Pieces::NONE,1,0,0,0));
                            //println!("black Bishop Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::b as u32,Pieces::NONE,0,0,0,0));
                            //println!("black Bishop moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
        }
    }
    // This methode will bassicly be the same for each slider piece
    pub fn generate_rook_moves(&mut self) {
        let mut bitboard:Bitboard;
        let mut atk:Bitboard;
        match self.board.side_to_move {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::R];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    // Cheking for betrayals .... am going insane
                    atk = self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::w]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::b],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::R as u32,Pieces::NONE,1,0,0,0));
                            //println!("white ROOK Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::R as u32,Pieces::NONE,0,0,0,0));
                            //println!("white Rook moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::r];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    atk = self.attacks.lookup_slider(SLIDER::ROOK,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::b]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // "DO NOT TRUST SHAPERD !!" 
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::w],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::r as u32,Pieces::NONE,1,0,0,0));
                            //println!("black Rook Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::r as u32,Pieces::NONE,0,0,0,0));
                            //println!("black Rook moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
        }
    }
    // This methode will bassicly be the same for each slider piece
    pub fn generate_queen_moves(&mut self) {
        let mut bitboard:Bitboard;
        let mut atk:Bitboard;
        match self.board.side_to_move {
            SIDES::WHITE => {
                bitboard = self.board.bitboards[Pieces::Q];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    // Cheking for betrayals .... am going insane
                    atk = self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::w]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::b],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::Q as u32,Pieces::NONE,1,0,0,0));
                            //println!("white Queen Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::Q as u32,Pieces::NONE,0,0,0,0));
                            //println!("white Queen moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
            SIDES::BLACK => {
                bitboard = self.board.bitboards[Pieces::q];
                if bitboard == 0 {
                    return;
                }
                let mut src:u8;
                while bitboard != 0 {
                    src = get_lsb(bitboard);
                    atk = self.attacks.lookup_slider(SLIDER::QUEEN,self.board.occupencies[COLOR::BOTH],src) & !self.board.occupencies[COLOR::b]; 
                    let mut dst:u8; 
                    while atk != 0 {
                        dst = get_lsb(atk);
                        // "DO NOT TRUST SHAPERD !!" 
                        // Checking if we are killing an enemy  
                        if get_bit!(self.board.occupencies[COLOR::w],dst) == 1 {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::q as u32,Pieces::NONE,1,0,0,0));
                            //println!("black Queen Captures from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        // else its just chilling 
                        else {
                            self.moves.add_move(encode_move!(src as u32,dst as u32,Pieces::q as u32,Pieces::NONE,0,0,0,0));
                            //println!("black Queen moves from {} to {}",SQUARE_NAME[src as usize],SQUARE_NAME[dst as usize]);
                        }
                        pop_bit!(atk,dst);
                    }
                    pop_bit!(bitboard,src);
                }
            }
        }
    }
}
