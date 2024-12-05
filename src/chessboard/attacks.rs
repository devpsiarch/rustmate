// here ze will define the attack masks for each piece so we can directly look them up when we need
// too in a rather fat way .
use super::bitboard::{Bitboard,get_lsb,bit_count};
use super::defs::{SIDES};
use crate::set_bit;
use crate::pop_bit;
use crate::get_bit;
use crate::magic::BISHOP_MAGICS;
use crate::magic::ROOK_MAGICS;
use crate::defs::SLIDER;
//these are the bitboards that represent the not X board , for example the not A file means its a
//bitboard where every bit is set execept the A file is not 
//theu are used for precalculating the attack masks 
//sorry for storing them in decimal
const NOT_A_FILE : Bitboard = 18374403900871474942;
const NOT_H_FILE : Bitboard = 9187201950435737471;
const NOT_GH_FILE: Bitboard = 4557430888798830399;
const NOT_AB_FILE: Bitboard = 18229723555195321596; 
/*
* these will be used later to calculate the indeces to index the attack tables 
* */
// Bishop relevant occupancy bits for each position
const BISHOP_ROB: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

// Rook relevant occupancy bits for each position
const ROOK_ROB: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

// this will hold all of our attack masks , this may be changed in the future 
// i willl add more pices attacks here each time we make one
// idk if thie will also hold the bishop , queen and rook tables ..
pub struct AttackMasks {
    pub pawn_attack_masks: [[Bitboard;64];2] ,
    pub king_attack_masks: [Bitboard;64] ,
    pub knight_attack_masks: [Bitboard;64] ,
     
    pub bishop_attack_mask: [Bitboard;64],           //We also store the premasks cuz we need them
    pub rook_attack_mask: [Bitboard;64],                // in indexing the tables and lookup
    pub bishop_attack_table: [[Bitboard;512];64],    //piece [squares][occupencies]
    pub rook_attack_table: [[Bitboard;4096];64],
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
* for slider pieces after getting the the possible attack maps , its left that we define a spicial
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
    // am gonna hanlde both rook and bishop in the same func , the consider true as bishop and
    // false as rook
    // we still need a methode too look them up after loading them
    fn load_slider_table(&mut self,piece:SLIDER){
        let mut attack_mask: Bitboard = 0;
        for i in 0..64 {
            // Here while doing the loop , we take advantage and at the same time we populate the
            // masks that there main use is just to indxe the tables that we accually need
            // i populated them in separate cases for speed , idk if that makes a diffrence 
            // and i dont want them to be changed when they dont need to 
            match piece {
                SLIDER::bishop => { 
                    self.bishop_attack_mask[i as usize] = get_bishop_attack_premask(i as u8); 
                    attack_mask = self.bishop_attack_mask[i as usize];
                }                
                SLIDER::rook => {
                    self.rook_attack_mask[i as usize] = get_rook_attack_premask(i as u8);
                    attack_mask = self.rook_attack_mask[i as usize];
                }
            }
            let bit_count = bit_count(attack_mask);
            let ocp_indecies = 1 << bit_count;
            for j in 0..ocp_indecies {
                let occ = set_occupency(j,attack_mask);
                // here wer perfome the "formula" that allows us to index each attack mask
                // using the magic numbers 
                match piece {
                    SLIDER::bishop => {
                        let magic_index = occ.wrapping_mul(BISHOP_MAGICS[i as usize].magic) >> (64 - BISHOP_ROB[i as usize]);
                        //storing the attack table for every possible occupency
                        self.bishop_attack_table[i as usize][magic_index as usize] = get_bishop_attack_otfmask(occ,i);
                    }
                    SLIDER::rook => {
                        let magic_index = occ.wrapping_mul(ROOK_MAGICS[i as usize].magic) >> (64 - ROOK_ROB[i as usize]);
                        //storing the attack table for every possible occupency
                        self.rook_attack_table[i as usize][magic_index as usize] = get_rook_attack_otfmask(occ,i);
                    }
                }
            }  
        } 
    }
    //this function here will looko up the pre caculated attack maps for slider pieces
    // WARNING ::: THIS METHODE EXPECTS THAT "load_slider_table" has been called before hand
    pub fn lookup_slider(&self,piece:SLIDER,occ:Bitboard,square:u8) -> Bitboard {
        let mut slider_attack:Bitboard = 0;
        // the lookup here is much faster and for the "index" variable , am keeping it there to
        // remind me of what the code originally do
        match piece {
            SLIDER::bishop => {
                let entry = &BISHOP_MAGICS[square as usize];
                let mut hash = occ & entry.mask;
                hash = hash.wrapping_mul(entry.magic) >> entry.shift;
                //let index = hash as usize + entry.offset;
                slider_attack = self.bishop_attack_table[square as usize][hash as usize];
            }
            SLIDER::rook => {
                let entry = &ROOK_MAGICS[square as usize];
                let mut hash = occ & entry.mask;
                hash = hash.wrapping_mul(entry.magic) >> entry.shift;
                //let index = hash as usize + entry.offset;
                slider_attack = self.rook_attack_table[square as usize][hash as usize];
            }
        }
        slider_attack
    }
    //this will call all of the above methodes to load the attack maps at once 
    pub fn new() -> Self {
        Self {
            pawn_attack_masks: [[0; 64]; 2], 
            king_attack_masks: [0; 64],
            knight_attack_masks: [0; 64],
            
            bishop_attack_mask:[0;64],
            rook_attack_mask:[0;64],
            bishop_attack_table: [[0;512];64],
            rook_attack_table: [[0;4096];64],
        }
    }
    pub fn load_attacks_maps(&mut self){
        self.load_pawn_masks();
        self.load_king_masks();
        self.load_knight_masks();
        self.load_slider_table(SLIDER::bishop);
        self.load_slider_table(SLIDER::rook);
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
    let mut mask: Bitboard = 0;
    // get the location of the current square
    let rank = square / 8;
    let file = square % 8;

    // ensure the rank and file are within bounds
    let min_rank = 1;
    let max_rank = 6;
    let min_file = 1;
    let max_file = 6;

    // getting the attack rays
    // Up-right diagonal
    for (r, f) in (rank + 1..=max_rank).zip(file + 1..=max_file) {
        set_bit!(mask, r * 8 + f);
    }
    // Down-right diagonal
    for (r, f) in (rank + 1..=max_rank).zip((min_file..file).rev()) {
        set_bit!(mask, r * 8 + f);
    }
    // Up-left diagonal
    for (r, f) in ((min_rank..rank).rev()).zip(file + 1..=max_file) {
        set_bit!(mask, r * 8 + f);
    }
    // Down-left diagonal
    for (r, f) in ((min_rank..rank).rev()).zip((min_file..file).rev()) {
        set_bit!(mask, r * 8 + f);
    }

    mask
}
//same thing here for the rook
pub fn get_rook_attack_premask(square: u8) -> Bitboard {
    let mut mask: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    // Ensure the rank and file are within bounds
    let min_rank = 1;
    let max_rank = 6;
    let min_file = 1;
    let max_file = 6;

    // Getting the attack rays (horizontal and vertical)

    // Rightward horizontal ray
    for f in file + 1..=max_file {
        set_bit!(mask, rank * 8 + f);
    }

    // Downward vertical ray
    for r in rank + 1..=max_rank {
        set_bit!(mask, r * 8 + file);
    }

    // Leftward horizontal ray
    for f in (min_file..file).rev() {
        set_bit!(mask, rank * 8 + f);
    }

    // Upward vertical ray
    for r in (min_rank..rank).rev() {
        set_bit!(mask, r * 8 + file);
    }

    mask
}
/*
* we have another version of the above functions that accually work perfectly given an occupency , 
* they keep tranversing until they hit a pieces , they are quite fast , but goind higher in the
* project , its better to index all of the possible maps to just look them up using the magic
* indecies rather then use the below function to get them one by one when looking for moves , but
* nevertheless we need them so here they are*/
pub fn get_bishop_attack_otfmask(block:Bitboard,square: u8) -> Bitboard {
    let mut mask: Bitboard = 0;
    // get the location of the current square
    let rank = square / 8;
    let file = square % 8;

    // ensure the rank and file are within bounds
    let min_rank = 0;
    let max_rank = 7;
    let min_file = 0;
    let max_file = 7;

    // getting the attack rays
    // Up-right diagonal
    for (r, f) in (rank + 1..=max_rank).zip(file + 1..=max_file) {
        set_bit!(mask, r * 8 + f);
        if (1 << r*8+f) & block != 0 {
            break;
        }
    }
    // Down-right diagonal
    for (r, f) in (rank + 1..=max_rank).zip((min_file..file).rev()) {
        set_bit!(mask, r * 8 + f);
        if (1 << r*8+f) & block != 0 {
            break;
        }
    }
    // Up-left diagonal
    for (r, f) in ((min_rank..rank).rev()).zip(file + 1..=max_file) {
        set_bit!(mask, r * 8 + f);
        if (1 << r*8+f) & block != 0 {
            break;
        }
    }
    // Down-left diagonal
    for (r, f) in ((min_rank..rank).rev()).zip((min_file..file).rev()) {
        set_bit!(mask, r * 8 + f);
        if (1 << r*8+f) & block != 0 {
            break;
        }
    }

    mask
}
//same thing here for the rook
pub fn get_rook_attack_otfmask(block:Bitboard,square: u8) -> Bitboard {
    let mut mask: Bitboard = 0;
    let rank = square / 8;
    let file = square % 8;

    // Ensure the rank and file are within bounds
    let min_rank = 0;
    let max_rank = 7;
    let min_file = 0;
    let max_file = 7;

    // Getting the attack rays (horizontal and vertical)

    // Rightward horizontal ray
    for f in file + 1..=max_file {
        set_bit!(mask, rank * 8 + f);
        if (1 << rank*8+f) & block != 0 {
            break;
        }
    }

    // Downward vertical ray
    for r in rank + 1..=max_rank {
        set_bit!(mask, r * 8 + file);
        if (1 << r*8+file) & block != 0 {
            break;
        }
    }

    // Leftward horizontal ray
    for f in (min_file..file).rev() {
        set_bit!(mask, rank * 8 + f);
        if (1 << rank*8+f) & block != 0 {
            break;
        }
    }

    // Upward vertical ray
    for r in (min_rank..rank).rev() {
        set_bit!(mask, r * 8 + file);
        if (1 << r*8+file) & block != 0 {
            break;
        }
    }

    mask
}
//this function below will help us get all the possible attack paterns of the slider pieces
//how exacly ? idk but we need it and also another version of mask attack for sliding pieces
//FUTURE ME :::: please check this is working fine 
pub fn set_occupency(index:u32,mut attack_map:Bitboard) -> Bitboard {
    let mut ocp :Bitboard = 0;
    let mut sqr:u8 = 0;
    let bit_mask = bit_count(attack_map);
    for i in 0..bit_mask {
        sqr = get_lsb(attack_map);
        pop_bit!(attack_map,sqr);
        if index & (1 << i) != 0 {
            ocp |= 1 << sqr;
        }
    }
    return ocp;
}
