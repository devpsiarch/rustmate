/*
* This file will contain the methodes for encoding and decoring the moves that are being "requested" , We also need more clerification for how the structure of the encoded Move will i think the best Idea is just to declare a type MOVE as u32 and have it to be a part of the move generation object and the generators to return Vecs of saif type 
* we need decoders and encoders 
* showers and methodes for for testing and seeing what are we coooking 
* */
/*
* We will encode moves in this manner below , 3 bytes for each
*/
/*
          binary move bits                               hexidecimal constants
    
    0000 0000 0000 0000 0011 1111    source square       0x00003f
    0000 0000 0000 1111 1100 0000    target square       0x000fc0
    0000 0000 1111 0000 0000 0000    piece               0x00f000
    0000 1111 0000 0000 0000 0000    promoted piece      0x0f0000
    0001 0000 0000 0000 0000 0000    capture flag        0x100000
    0010 0000 0000 0000 0000 0000    double push flag    0x200000
    0100 0000 0000 0000 0000 0000    enpassant flag      0x400000
    1000 0000 0000 0000 0000 0000    castling flag       0x800000
*/
use crate::chessboard::defs::{SQUARE_NAME,UNICODE_PIECES};
pub type Move = u32;             // sure 4 bits unused is better then nothing

pub struct MoveMask;
impl MoveMask {
    pub const SRC:Move                = 0x3f ;
    pub const DST:Move                = 0xfc0;
    pub const PIECE:Move              = 0xf000;
    pub const PROMOTION:Move          = 0xf0000;
    pub const CAPTURE_FLAG:Move       = 0x100000;
    pub const DOUBLE_JUMP_FLAG:Move   = 0x200000;
    pub const EN_PASSANT_FLAG:Move    = 0x400000;
    pub const CASTLE_FLAG:Move        = 0x800000;
}


// I will use macros to decode and encode the moves , IDK if this will be faster but looking at it
// , i could use absolutly nothing so in that case am using macros 
// am not handling jack ass here , why ? no one will use this but me wytb ?
#[macro_export]
macro_rules! encode_move {
    ($src:expr, $dst:expr, $piece:expr, $promotion:expr, $capture:expr, $double_jump:expr, $enpassant:expr, $castle:expr) => {
        ($src) | ($dst << 6) | ($piece << 12) | ($promotion << 16) | ($capture << 20) | ($double_jump << 21)
        | ($enpassant << 22) | ($castle << 23) 
    };
}
/*
* Here in contrast we shift because we want exact infomation about the MOVE
*/
#[macro_export]
macro_rules! get_move_src {
    ($mv:expr) => (
        ($mv & MoveMask::SRC)
    )
}
#[macro_export]
macro_rules! get_move_dst {
    ($mv:expr) => (
        ($mv & MoveMask::DST) >> 6
    )
}
#[macro_export]
macro_rules! get_move_piece {
    ($mv:expr) => (
        ($mv & MoveMask::PIECE) >> 12
    )
}
#[macro_export]
macro_rules! get_move_promotion {
    ($mv:expr) => (
        ($mv & MoveMask::PROMOTION) >> 16
    )
}
/*
* In the other macros that check for just one bit , we dont need to shift , we just need to check
* if that macros returns a number (the bit is set) or returns a 0 (bit is not set)
* IDK i thought i would waste that time on shifting 
*/

#[macro_export]
macro_rules! get_move_capture {
    ($mv:expr) => (
        $mv & MoveMask::CAPTURE_FLAG
    )
}
#[macro_export]
macro_rules! get_move_doublejump {
    ($mv:expr) => (
        $mv & MoveMask::DOUBLE_JUMP_FLAG
    )
}
#[macro_export]
macro_rules! get_move_enpassant {
    ($mv:expr) => (
        $mv & MoveMask::EN_PASSANT_FLAG
    )
}
#[macro_export]
macro_rules! get_move_castle {
    ($mv:expr) => (
        $mv & MoveMask::CASTLE_FLAG
    )
}
// Tread this as the print bitboard function , no methode no nohing just a helper that you will
// only use once or twice to test then complety forget about it 
#[allow(dead_code)]
pub fn show_move(mv:Move) {
    let src = get_move_src!(mv);
    let dst = get_move_dst!(mv);
    let p = get_move_piece!(mv);
    let promo = get_move_promotion!(mv);
    let cap = get_move_capture!(mv);
    let d = get_move_doublejump!(mv);
    let enpassant = get_move_enpassant!(mv);
    let cast = get_move_castle!(mv);
    println!("move src {}",SQUARE_NAME[src as usize]);
    println!("move dst {}",SQUARE_NAME[dst as usize]);
    println!("move piece {}",UNICODE_PIECES[p as usize]);
    println!("move promo piece {}",UNICODE_PIECES[promo as usize]);
    if cap != 0 {
        println!("move capture");
    } else {
        println!("move no capture");
    }
    if d != 0 {
        println!("move double jump");
    } else {
        println!("move No double jump");
    }
    if enpassant != 0 {
        println!("move enpassant");
    } else {
        println!("move no enpassant");
    }
    if cast != 0 {
        println!("move castle");
    } else {
        println!("move no caste");
    }
}
