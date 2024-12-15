/*
* This object will store the moves as integers in the heap , i just dont want them laying around in
* the stack so ill use the Vec to store them and create few methodes to fix em up
*/
use crate::movegen::movecode::{Move};
use crate::{
    get_move_dst,
    get_move_src,
    get_move_capture,
    get_move_castle,
    get_move_promotion,
    get_move_doublejump,
    get_move_enpassant,
};
use crate::MOVE_MASK;
use crate::chessboard::defs::{SQUARE_NAME,UNICODE_PIECES,Pieces};
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
    // Pretty prints the moves , this is only for me to not go absolutly insane while trouble
    // shooting
    pub fn print_all_moves(&self) {
        println!(
        "{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}",
        "move", "piece", "capture", "double", "enpass", "castling"
        );
        for i in 0..self.Count {
            let promoted = if get_move_promotion!(self.List[i]) == Pieces::NONE {"NONE"} else {UNICODE_PIECES[get_move_promotion!(self.List[i]) as usize]};
            println!("{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}",
                SQUARE_NAME[get_move_src!(self.List[i]) as usize].to_owned()+SQUARE_NAME[get_move_dst!(self.List[i]) as usize]
                ,promoted
                ,get_move_capture!(self.List[i])
                ,get_move_doublejump!(self.List[i])
                ,get_move_enpassant!(self.List[i])
                ,get_move_castle!(self.List[i]));
        }
        println!("\n\t\t\tTotal moves : {}",self.Count);
    }
}
