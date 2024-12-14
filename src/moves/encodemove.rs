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
use crate::chessboard::defs::{Pieces};
pub type Move = u32;             // sure 4 bits unused is better then nothing
type Move_decoder = fn(m:Move) -> u8;
const ENCODING_SIZE: usize = 8;

pub struct MOVE_MASK;
impl MOVE_MASK {
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
#[macro_export]
macro_rules! get_move_src {
    ($mv:expr) => (
        ($mv & MOVE_MASK::SRC)
    )
}
#[macro_export]
macro_rules! get_move_dst {
    ($mv:expr) => (
        ($mv & MOVE_MASK::DST) >> 6
    )
}
#[macro_export]
macro_rules! get_move_piece {
    ($mv:expr) => (
        ($mv & MOVE_MASK::PIECE) >> 12
    )
}
#[macro_export]
macro_rules! get_move_promotion {
    ($mv:expr) => (
        ($mv & MOVE_MASK::PROMOTION) >> 16
    )
}
#[macro_export]
macro_rules! get_move_capture {
    ($mv:expr) => (
        ($mv & MOVE_MASK::CAPTURE_FLAG) >> 20
    )
}
#[macro_export]
macro_rules! get_move_doublejump {
    ($mv:expr) => (
        ($mv & MOVE_MASK::DOUBLE_JUMP_FLAG) >> 21
    )
}
#[macro_export]
macro_rules! get_move_enpassant {
    ($mv:expr) => (
        ($mv & MOVE_MASK::EN_PASSANT_FLAG) >> 22
    )
}
#[macro_export]
macro_rules! get_move_castle {
    ($mv:expr) => (
        ($mv & MOVE_MASK::CASTLE_FLAG) >> 23
    )
}
