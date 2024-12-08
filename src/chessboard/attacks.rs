use super::bitboard::{Bitboard,bit_count};
use crate::defs::{SLIDER,SIDES};
use crate::magic::{ROOK_MAGICS,BISHOP_MAGICS,ROOK_MAP_SIZE,BISHOP_MAP_SIZE};
use crate::chessboard::atkgen::{
    get_pawn_attack_mask,get_king_attack_mask,get_knight_attack_mask,
    get_bishop_attack_premask,get_bishop_attack_otfmask,
    get_rook_attack_premask,get_rook_attack_otfmask,
    set_occupency
};
// this will hold all of our attack masks , this may be changed in the future 
// i willl add more pices attacks here each time we make one
// idk if thie will also hold the bishop , queen and rook tables ..
pub struct AttackMasks {
    pub pawn_attack_masks: [[Bitboard;64];2] ,
    pub king_attack_masks: [Bitboard;64] ,
    pub knight_attack_masks: [Bitboard;64] ,
    pub bishop_attack_table: Vec<Bitboard>,    //piece [squares][occupencies]
    pub rook_attack_table: Vec<Bitboard>,
}

// this will have all the methodes that will allow for loading the attack masks
//I shouuld probibly have a methode that calls all the other methodes to load all the attack maps
impl AttackMasks {
    //this will call all of the above methodes to load the attack maps at once 
    pub fn new() -> Self {
        Self {
            pawn_attack_masks: [[0; 64]; 2], 
            king_attack_masks: [0; 64],
            knight_attack_masks: [0; 64],
            bishop_attack_table: vec![0;BISHOP_MAP_SIZE],
            rook_attack_table: vec![0;ROOK_MAP_SIZE],
        }
    }
    pub fn load_attacks_maps(&mut self){
        self.load_pawn_masks();
        self.load_king_masks();
        self.load_knight_masks();
        self.load_slider_table(SLIDER::BISHOP);
        self.load_slider_table(SLIDER::ROOK);
    }
    //These shall stays private ok future me ?
    fn load_pawn_masks(&mut self){
        //loop thought all the squares and populate the attack maps for black and white 
        //cur they are pawns 
        for i in 0..64 {
            // 0 => white || 1 => black  
            self.pawn_attack_masks[0][i] = get_pawn_attack_mask(i.try_into().unwrap(),SIDES::WHITE);
            self.pawn_attack_masks[1][i] = get_pawn_attack_mask(i.try_into().unwrap(),SIDES::BLACK);
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
        let mut attack_mask: Bitboard;
        for i in 0..64 {
            // Here while doing the loop , we take advantage and at the same time we populate the
            // masks that there main use is just to indxe the tables that we accually need
            // i populated them in separate cases for speed , idk if that makes a diffrence 
            // and i dont want them to be changed when they dont need to 
            match piece {
                SLIDER::BISHOP => { 
                    attack_mask = get_bishop_attack_premask(i as u8); 
                }                
                SLIDER::ROOK => {
                    attack_mask = get_rook_attack_premask(i as u8); 
                }
                // We dont load any queen slider movement because we dont have too since queens
                // moves are a combination between rook and bishop , we store them and when looking
                // up a move for a queen we OR them and return .
                SLIDER::QUEEN => {
                    panic!("Can't load slider QUEEN");
                } 
            }
            let bit_count = bit_count(attack_mask);
            let ocp_indecies = 1 << bit_count;
            for j in 0..ocp_indecies {
                let occ = set_occupency(j,attack_mask);
                // Here we use the methode prebuilt 
                match piece {
                    SLIDER::BISHOP => {
                        let index = BISHOP_MAGICS[i as usize].get_index(occ); 
                        self.bishop_attack_table[index] = get_bishop_attack_otfmask(occ,i);
                    }
                    SLIDER::ROOK => {
                        let index = ROOK_MAGICS[i as usize].get_index(occ); 
                        self.rook_attack_table[index] = get_rook_attack_otfmask(occ,i);
                    }
                    // Same here , we dont care about the queen since its the others combined 
                    SLIDER::QUEEN => {
                        panic!("Can't load slider QUEEN");
                    }
                }
            }  
        } 
    }
    //this function here will looko up the pre caculated attack maps for slider pieces
    // WARNING ::: THIS METHODE EXPECTS THAT "load_slider_table" has been called before hand
    pub fn lookup_slider(&self,piece:SLIDER,occ:Bitboard,square:u8) -> Bitboard {
        // remind me of what the code originally do
        // we handle each slider alone here even for qeen with is bascilly the both or bishop and 
        match piece {
            SLIDER::BISHOP => {
                let entry = &BISHOP_MAGICS[square as usize];
                self.bishop_attack_table[entry.get_index(occ)]
            }
            SLIDER::ROOK => {
                let entry = &ROOK_MAGICS[square as usize];
                self.rook_attack_table[entry.get_index(occ)]
            }
            SLIDER::QUEEN => {
                // doing the same twice doesnt seem to work so check this out
                let r_index = &ROOK_MAGICS[square as usize].get_index(occ); 
                let b_index = &BISHOP_MAGICS[square as usize].get_index(occ); 
                self.rook_attack_table[*r_index] ^ self.bishop_attack_table[*b_index]
            }
        }
    }
}
