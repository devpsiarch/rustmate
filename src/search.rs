/* 
* We seach for moves in this module , well do all sort of ways to do so ...
* 1- random move generator <baby step>
* */
use crate::movegen::MoveGenerator;
use crate::Chessboard;
use crate::attacks::AttackMasks;
use crate::movegen::movecode::Move;

pub fn search_move(board:&mut Chessboard,atk:&AttackMasks,depth:u32) -> Move{
    let mut generator = MoveGenerator::new(board,atk);
    generator.generate_moves();
    generator.moves.list[depth as usize] 
}
