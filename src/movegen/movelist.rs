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
    get_move_piece,
    get_move_promotion,
    get_move_doublejump,
    get_move_enpassant,
};
use crate::MoveMask;
use crate::chessboard::defs::{SQUARE_NAME,UNICODE_PIECES,Pieces};
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
    // Pretty prints the moves , this is only for me to not go absolutly insane while trouble
    // shooting
    #[allow(dead_code)]
    pub fn print_all_moves(&self) {
        println!(
        "{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}{:<12}",
        "move", "piece", "promoted", "capture", "double", "enpass", "castling"
        );
        for i in 0..self.count {
            let promoted = if get_move_promotion!(self.list[i]) == Pieces::NONE {"NONE"} 
            else {UNICODE_PIECES[get_move_promotion!(self.list[i]) as usize]};
           
            println!("{:<4}{:<10}{:<10}{:<12}{:<12}{:<12}{:<12}{:<12}",
                i
                ,SQUARE_NAME[get_move_src!(self.list[i]) as usize].to_owned()+SQUARE_NAME[get_move_dst!(self.list[i]) as usize]
                ,UNICODE_PIECES[get_move_piece!(self.list[i]) as usize]
                ,promoted
                ,get_move_capture!(self.list[i])
                ,get_move_doublejump!(self.list[i])
                ,get_move_enpassant!(self.list[i])
                ,get_move_castle!(self.list[i]));
        }
        println!("\n\t\t\tTotal moves : {}",self.count);
    }
}
