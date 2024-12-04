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
#[macro_export]
macro_rules! pop_bit {
    ($board:expr,$bit:expr) => {
        if get_bit!($board,$bit) != 0 {
            $board ^= (1 << $bit) 
        }
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
    //try to remember to remove this later i need it rn to check methodes are working fine
    println!("A B C D E F G H");
}

pub fn bit_count(board:Bitboard) -> u8{
    let mut count:u8 = 0; 
    let mut temp = board;
    while temp != 0 {
        temp &= temp -1;
        count += 1;
    }  
    count
}
pub fn get_lsb(board:Bitboard) -> u8 {
    let mut temp = board;
    if temp == 0 {
        panic!("supplied zero bitboard to get tsb !!!");
    }
    else{
        temp = (temp & temp.wrapping_neg()) - 1;   
    }
    bit_count(temp)
}
