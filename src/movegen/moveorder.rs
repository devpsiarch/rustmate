
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

use crate::Move;
use crate::evalu::defs::PIECES_COST;

// In the make move function we have to handle the "Horizon effect" althou i am not very familliar
// with said effect but hey
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum move_type {
    ALL_MOVES,
    CAPTURE_MOVE,
}


impl<'a> MoveGenerator<'a> {
    pub fn evaluate_move(&self,_some_move:Move) -> i32{
        // for now evaluate based on MVV-LVA
        if get_move_capture!(_some_move) != 0{
            let mut victim_value = 0;
            let mut agressor_value = 0;
            let dst = get_move_dst!(_some_move);
            let src = get_move_src!(_some_move);

            for i in Pieces::P..=Pieces::k {
                // match the bitboards with there dst 
                // if they match get the evalution from the table of pices costs
                if self.board.bitboards[i] & (1 << dst) != 0 {
                    victim_value = PIECES_COST[i];
                }
                if self.board.bitboards[i] & ( 1 << src) != 0{
                    agressor_value = PIECES_COST[i];
                }
                if agressor_value != 0 && victim_value != 0{
                    break;
                }
            }
            // we reverse to make the white position (since maximizer) and vice-verca
            return -(victim_value * 10 - agressor_value);
        }else{
            return 0;
        }
    }

    // a function that sorts the moves that have been generated
    pub fn move_order(&mut self){
        let mut stolen_list = std::mem::take(&mut self.moves.list);
        
        stolen_list.sort_by(|a,b| self.evaluate_move(*b).cmp(&self.evaluate_move(*a)));
        
        self.moves.list = stolen_list;
    }
}
