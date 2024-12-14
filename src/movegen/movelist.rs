/*
* This object will store the moves as integers in the heap , i just dont want them laying around in
* the stack so ill use the Vec to store them and create few methodes to fix em up
*/
use crate::movegen::movecode::{Move};
use crate::{
    get_move_dst,
    get_move_src,
    get_move_promotion,
};
use crate::MOVE_MASK;
use crate::chessboard::defs::{SQUARE_NAME,UNICODE_PIECES};
const MAX_LEGAL_MOVES:usize = 256;  

pub struct MoveList {
    List: Vec<Move>,
    Count:usize,
}
impl MoveList {
    pub fn new() -> Self {
        Self {
            List:Vec::with_capacity(MAX_LEGAL_MOVES),
            Count:0,
        }
    }
    pub fn add_move(&mut self,mv:Move) {
        self.List.push(mv);
        self.Count += 1;
    }
    pub fn print_all_moves(&self) {
        for i in 0..self.Count {
            println!("{}{}{}",
                SQUARE_NAME[get_move_src!(self.List[i]) as usize]
                ,SQUARE_NAME[get_move_dst!(self.List[i]) as usize]
                ,UNICODE_PIECES[get_move_promotion!(self.List[i]) as usize]);
        }    
    }
}
