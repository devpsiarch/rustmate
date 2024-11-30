// here ze will define the attack masks for each piece so we can directly look them up when we need
// too in a rather fat way .
use super::bitboard::{Bitboard};
use super::defs::{SIDES};
use crate::set_bit;

//these are the bitboards that represent the not X board , for example the not A file means its a
//bitboard where every bit is set execept the A file is not 
//theu are used for precalculating the attack masks 
//sorry for storing them in decimal
const NOT_A_FILE : Bitboard = 18374403900871474942;
const NOT_H_FILE : Bitboard = 9187201950435737471;
const NOT_GH_FILE: Bitboard = 4557430888798830399;
const NOT_AB_FILE: Bitboard = 18229723555195321596; 
// this will hold all of our attack masks , this may be changed in the future 
// i willl add more pices attacks here each time we make one
// idk if thie will also hold the bishop , queen and rook tables ..
pub struct AttackMasks {
    pub pawn_attack_masks: [[Bitboard;64];2] ,
    pub king_attack_masks: [Bitboard;64] ,
    pub knight_attack_masks: [Bitboard;64] ,
}
// this will have all the methodes that will allow for loading the attack masks
//I shouuld probibly have a methode that calls all the other methodes to load all the attack maps
impl AttackMasks {
    //These shall stays private ok future me ?
    fn load_pawn_masks(&mut self){
        //loop thought all the squares and populate the attack maps for black and white 
        //cur they are pawns 
        for i in 0..64 {
            // 0 => white || 1 => black  
            self.pawn_attack_masks[0][i] = get_pawn_attack_mask(i.try_into().unwrap(),SIDES::white);
            self.pawn_attack_masks[1][i] = get_pawn_attack_mask(i.try_into().unwrap(),SIDES::black);
        }
    }
    fn load_king_masks(&mut self){
        //the same as other function just for king ans its color neutral
        for i in 0..64 {
            self.king_attack_masks[i] = get_king_attack_mask(i.try_into().unwrap());
        }
    }
    fn load_knight_masks(&mut self){
        //the same as other function just for king ans its color neutral
        for i in 0..64 {
            self.knight_attack_masks[i] = get_knight_attack_mask(i.try_into().unwrap());
        }
    }
    //this will call all of the above methodes to load the attack maps at once 
    pub fn new() -> Self {
        Self {
            pawn_attack_masks: [[0; 64]; 2], 
            king_attack_masks: [0; 64],
            knight_attack_masks: [0; 64],
        }
    }
    pub fn load_attacks_maps(&mut self){
        self.load_pawn_masks();
        self.load_king_masks();
        self.load_knight_masks();
    }
}
// this returns a bitboard of all the possible attacks a pawn can have
pub fn get_pawn_attack_mask(square: u8,color: SIDES) -> Bitboard {
    //we declare the mask that we are gonna return 
    let mut mask : Bitboard = 0;
    // we declare this temp to hold the value of 1 theat we wanna add , it wont work otherwise
    let mut temp: Bitboard = 0;
    set_bit!(temp,square);
    match color {
        //in the case for each color we check if the attack possition is out of bounds
        //by anding the not a/h file to check
        SIDES::white => {
            if NOT_A_FILE & temp >> 7 != 0 {
                mask |= temp >> 7;
        }
            if NOT_H_FILE & temp >> 9 != 0 {
                mask |= temp >> 9;
            }
        }
        SIDES::black => {
            if NOT_H_FILE & temp << 7 != 0 {
                mask |= temp << 7;
            }
            if NOT_A_FILE & temp << 9 != 0 {
                mask |= temp << 9;
            }
        }
    }
    mask
}
//this does the same for the kings pieces
pub fn get_king_attack_mask(square: u8) -> Bitboard {
    //we declare the mask that we are gonna return 
    let mut mask : Bitboard = 0;
    // we declare this temp to hold the value of 1 theat we wanna add , it wont work otherwise
    let mut temp: Bitboard = 0;
    set_bit!(temp,square);
    // we dont need to worry about the up and the down squares because they wont overlap
    //up square
    mask |= temp >> 8;
    if (temp >> 1) & NOT_H_FILE != 0 {
        mask |= temp >> 1;
    }
    if (temp >> 9) & NOT_H_FILE != 0 {
        mask |= temp >> 9;
    }
    if (temp >> 7) & NOT_A_FILE != 0 {
        mask |= temp >> 7;
    }

    //down square
    mask |= temp << 8;
    if (temp << 1) & NOT_A_FILE != 0 {
        mask |= temp << 1;
    }
    if (temp << 9) & NOT_A_FILE != 0 {
        mask |= temp << 9;
    }
    if (temp << 7) & NOT_H_FILE != 0 {
        mask |= temp << 7;
    } 
    mask
}
//the same is done for this one as well , this time for the knight
pub fn get_knight_attack_mask(square: u8) -> Bitboard {
    //we declare the mask that we are gonna return 
    let mut mask : Bitboard = 0;
    // we declare this temp to hold the value of 1 theat we wanna add , it wont work otherwise
    let mut temp: Bitboard = 0;
    set_bit!(temp,square);
    // and this is for shifts (it really is easy when you have dont this before)
    if (temp >> 6) & NOT_AB_FILE != 0 {
        mask |= temp >> 6;
    }
    if (temp >> 15) & NOT_A_FILE != 0 {
        mask |= temp >> 15;
    }

    if (temp >> 17) & NOT_H_FILE != 0 {
        mask |= temp >> 17;
    }
    if (temp >> 10) & NOT_GH_FILE != 0 {
        mask |= temp >> 10;
    }

    // Downward shifts (left shifts)
    if (temp << 6) & NOT_GH_FILE != 0 {
        mask |= temp << 6;
    }
    if (temp << 15) & NOT_H_FILE != 0 {
        mask |= temp << 15;
    }

    if (temp << 17) & NOT_A_FILE != 0 {
        mask |= temp << 17;
    }
    if (temp << 10) & NOT_AB_FILE != 0 {
        mask |= temp << 10;
    }
    mask
}
