pub type Bitboard = u64;

#[macro_export]
macro_rules! set_bit {
    ($board:expr,$bit:expr) => {
        (($board) |= (1 << $bit)) 
    };
}

#[macro_export]
macro_rules! get_bit {
    ($board:expr,$bit:expr) => {
        if ($board & (1 << $bit)) != 0 {
            1
        } else {
            0
        }
    };
}
#[macro_export]
macro_rules! kill_board {
    ($board:expr) => {
        (($board) = 0) 
    };
}
#[allow(dead_code)]
pub fn print_bitboard(board : &Bitboard){
    for i in 0..8 {
        for j in 0..8 {
            print!("{} ",get_bit!(board,i*8+j));
        }
        print!("\n");
    }
    print!("\n");
}
