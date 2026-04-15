/*
* This object will store the moves as integers in the heap , i just dont want them laying around in
* the stack so ill use the Vec to store them and create few methodes to fix em up
*/
use crate::movegen::movecode::{Move};

const MAX_LEGAL_MOVES:usize = 256;  

pub struct MoveList {
    pub list: Vec<Move>,
    pub count:usize,
}
impl MoveList {
    pub fn new() -> Self {
        Self {
            list:Vec::with_capacity(MAX_LEGAL_MOVES),
            count:0,
        }
    }
    pub fn add_move(&mut self,mv:Move) {
        self.list.push(mv);
        self.count += 1;
    }

    
}
