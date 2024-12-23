use crate::movegen::MoveGenerator; 
use crate::chessboard::{Chessboard};
use crate::chessboard::attacks::{AttackMasks};
use crate::move_type; 

use crate::{
    get_move_capture,
    get_move_castle,
    get_move_promotion,
    get_move_doublejump,
    get_move_enpassant,
};
use crate::MoveMask;
pub fn perf_driver(mut board:Chessboard,atk:AttackMasks,depth:u32) -> (u64,u64,u64,u64,u64) {
    if depth == 0 {
        return (1,0,0,0,0);
    }
    // Creating a generator object
    let mut generator = MoveGenerator::new(&mut board,&atk);  
    generator.generate_moves();
    let mut nodes:u64 = 0;
    let mut captures:u64 = 0;
    let mut ep:u64 = 0;
    let mut casltes:u64 = 0;
    let mut promos:u64 = 0;
    for i in 0..generator.moves.count {
        let copy = generator.board.clone();
        if get_move_capture!(generator.moves.list[i]) != 0 {
            captures += 1;
        }
        if get_move_enpassant!(generator.moves.list[i]) != 0 {
            ep += 1;
        }
        if get_move_castle!(generator.moves.list[i]) != 0 {
            casltes += 1;
        }
        if get_move_promotion!(generator.moves.list[i]) != 0 {
            promos += 1;
        }
        if generator.make_move(generator.moves.list[i],move_type::ALL_MOVES) == true {
        let (new_nodes, new_captures, new_ep, new_castles, new_promos) = 
            perf_driver(generator.board.clone(), generator.attacks.clone(), depth - 1);

            nodes += new_nodes;
            captures += new_captures;
            ep += new_ep;
            casltes += new_castles;
            promos += new_promos;
        }else{
           continue; 
        }
        generator.board.restore_board(copy);
    }
    (nodes,captures,ep,casltes,promos)
}
