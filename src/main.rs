type Bitboard = u64;

macro_rules! set_bit {
    ($board:expr,$bit:expr) => {
        (($board) |= (1 << $bit)) 
    };
}

macro_rules! get_bit {
    ($board:expr,$bit:expr) => {
        if ($board & (1 << $bit)) != 0 {
            1
        } else {
            0
        }
    };
}

macro_rules! kill_board {
    ($board:expr) => {
        (($board) = 0) 
    };
}

fn print_bitboard(board : &Bitboard){
    for i in 0..8 {
        for j in 0..8 {
            print!("{} ",get_bit!(board,i*8+j));
        }
        print!("\n");
    }
}
//we define this to identify eche board in bitboards contained by Chessboard 
pub struct Pieces;
#[allow(non_upper_case_globals)]
impl Pieces {
    pub const P: usize = 0; // White Pawn
    pub const R: usize = 1; // White Rook
    pub const N: usize = 2; // White Knight
    pub const B: usize = 3; // White Bishop
    pub const Q: usize = 4; // White Queen
    pub const K: usize = 5; // White King

    pub const p: usize = 6; // Black Pawn
    pub const r: usize = 7; // Black Rook
    pub const n: usize = 8; // Black Knight
    pub const b: usize = 9; // Black Bishop
    pub const q: usize = 10; // Black Queen
    pub const k: usize = 11; // Black King
}

const UNICODE_PIECES: [&str; 12] = [
    "♟︎", "♞", "♝", "♜", "♛", "♚",
    "♙", "♘", "♗", "♖", "♕", "♔"
];

struct Chessboard {
    bitboards : [Bitboard;12],
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI 
    fn new() -> Chessboard {
        let board = Chessboard {
            bitboards : [0;12], 
        }; 
        board
    }
    fn print_chessboard(&self){
        //this assumes that the input Chessboard is correct such that no 2 Pieces occupy the same
        //square

        for r in 0..8 {
            for c in 0..8 {
                if c == 0 {
                    print!("{} |",8-r);
                }
                let square = r*8+c;
                let mut fail : u32 = 0;
                for i in 0..12 {
                    if get_bit!(self.bitboards[i],r*8+c) == 1 {
                        print!("{} ",UNICODE_PIECES[i]);
                        fail+=1;
                    }
                }
                if fail == 0 {
                    print!("{} ",0);
                }
            }
            print!("\n");
        }
        print!("   A B C D E F G H\n");
    }
}


fn main() {
    let mut chess = Chessboard::new();
    for i in 0..8 {
        set_bit!(chess.bitboards[Pieces::p],i);
    }
    for i in 8..16 {
        set_bit!(chess.bitboards[Pieces::P],i);
    }
    chess.print_chessboard();
}
