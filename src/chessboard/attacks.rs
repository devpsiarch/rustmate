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

/*
* In this aproches we are gonna encode all the possible attack paterns of each pieces to make for
* easier calculations , in the case for leaper pieces : knight , pawn , king its very straight
* forward on how we can do that , and when there is a blocking pieces we can just perfrom some bit
* operations and get the map we need 
* in the case for the other sliding pieces , we can do the same so , out main objective is to get
* all the possible attack maps for each case of blocking , again this is gonna be worth it in the
* long run beacause the look up is far faster then to just loop around when making moves .
*
* for leaper pieces after getting the the possible attack maps , its left that we define a spicial
* indexing methodes to find the corresponding map , kinda like creating out own hash map
* */
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
fn get_pawn_attack_mask(square: u8,color: SIDES) -> Bitboard {
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
fn get_king_attack_mask(square: u8) -> Bitboard {
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
fn get_knight_attack_mask(square: u8) -> Bitboard {
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
// this func will get up the attack mask the rook , but not quite ...
// it is no complete and this would be used to get the accual maps
pub fn get_bishop_attack_premask(square: u8) -> Bitboard {
    let mut mask:Bitboard = 0;
    //get the location of the current square  
    let rank = square/8;
    let file = square%8;
    //getting the attack rays 
    //we skip the last one before hiting the edge of the board ... idk why ask chess wiki
    for (r,f) in (rank+1..=6).zip(file+1..=6) {
        set_bit!(mask,r*8+f);
    }
    for (r,f) in (rank+1..=6).zip((1..=file-1).rev()) {
        set_bit!(mask,r*8+f);
    }
    for (r,f) in ((1..=rank-1).rev()).zip(file+1..=6) {
        set_bit!(mask,r*8+f);
    }
    for (r,f) in ((1..=rank-1).rev()).zip((1..=file-1).rev()) {
        set_bit!(mask,r*8+f);
    }
    mask
}
//same thing here for the rook
pub fn get_rook_attack_premask(square: u8) -> Bitboard {
    let mut mask:Bitboard = 0;
    let rank = square/8;
    let file = square%8;
    //setting the rays
    for f in file+1..=6 {
        set_bit!(mask,rank*8+f);
    }
    for r in rank+1..=6 {
        set_bit!(mask,r*8+file);
    }
    for f in (1..=file-1).rev() {
        set_bit!(mask,rank*8+f);
    }
    for r in (1..=rank-1).rev() {
        set_bit!(mask,r*8+file);
    }
    mask
}
