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
    "♟︎", "♜" ,"♞" ,"♝", "♛", "♚",
    "♙", "♖" ,"♘", "♗", "♕", "♔"
];
const SPLITTER: char = '/';
const DASH: char = '-';
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";

struct Chessboard {
    bitboards : [Bitboard;12],
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI , the new function takes the fen string else it remains as it is 
    fn new() -> Chessboard {
        let board = Chessboard {
            bitboards : [0;12], 
        }; 
        board
    }
    fn load_fen(&mut self,fen:&str) -> bool{
        let sections : Vec<&str> = fen.split(" ").collect();
        //each function that is used to parse the FEN string is gonna be checked if anything bad
        //happens ... untile i find a rustier way of doing this 
        if !self.load_board(sections[0]) {
            panic!("Error parsing the board pieces");
        }else {
            false
        }
    }
    //i will impliment 6 part to parse the fen string (board , side to move , castle ...)
    // part 1 : board
    fn load_board(&mut self,part:&str) -> bool {
        //we use these to keep track of the square we are in
        let mut rank : u8 = 0;
        let mut file : u8 = 0;
        // i use this to handle errors 
        let mut result_code = true;

        for c in part.chars(){
            let square = rank*8 + file;
            match c {
                'P' =>   set_bit!(self.bitboards[Pieces::P],square), 
                'R' =>   set_bit!(self.bitboards[Pieces::R],square), 
                'N' =>   set_bit!(self.bitboards[Pieces::N],square), 
                'B' =>   set_bit!(self.bitboards[Pieces::B],square), 
                'Q' =>   set_bit!(self.bitboards[Pieces::Q],square), 
                'K' =>   set_bit!(self.bitboards[Pieces::K],square), 
                'p' =>   set_bit!(self.bitboards[Pieces::p],square),
                'r' =>   set_bit!(self.bitboards[Pieces::r],square), 
                'n' =>   set_bit!(self.bitboards[Pieces::n],square), 
                'b' =>   set_bit!(self.bitboards[Pieces::b],square), 
                'q' =>   set_bit!(self.bitboards[Pieces::q],square), 
                'k' =>   set_bit!(self.bitboards[Pieces::k],square), 
                
                '1'..='8' => {
                    if let Some(x) = c.to_digit(10) {
                        file += x as u8;
                    }
                } 
                
                SPLITTER => {
                    result_code = file == 8;
                    rank += 1;
                    file = 0;
                }
                
                _ => result_code = false,
            }
            if LIST_OF_PIECES.contains(c) {
                file += 1;
            }
            if !result_code {
                break;
            }
        }
        result_code
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
    chess.load_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    chess.print_chessboard();
    
}
