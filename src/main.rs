type Bitboard = u64;
type Fen_result = Result<(),u8>;
type Fen_parser = fn(board:&mut Chessboard,part:&str) -> bool;

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
    
    pub const white : bool = true;
    pub const black : bool = false;
}

const UNICODE_PIECES: [&str; 12] = [
    "♟︎", "♜" ,"♞" ,"♝", "♛", "♚",
    "♙", "♖" ,"♘", "♗", "♕", "♔"
];
const SPLITTER: char = '/';
const DASH: char = '-';
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const LEN_FEN_STRING: usize = 6;

#[derive(Clone)]
enum SIDES {
    white,
    black,
}
#[derive(Clone)]
pub struct Chessboard {
    bitboards : [Bitboard;12],
    side_to_mode : SIDES,
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI , 
    fn new() -> Self {
        Self {
            bitboards : [0;12], 
            side_to_mode : SIDES::white, 
        } 
    }
    fn reset(&mut self) {
        self.bitboards = [0;12];
        self.side_to_mode = SIDES::white;
    } 
    fn parse_fen(&mut self,fen:&str) -> Fen_result {
        let fen_parts: Vec<&str> = fen.split(" ").collect();
        //checks the size of the fen parts there always should be 6 
        let n_fen_parts_ok = fen_parts.len() == LEN_FEN_STRING; 
        let mut result : Fen_result = if n_fen_parts_ok {
            Ok(())
        }else{
            Err(0)
        };
        if n_fen_parts_ok {
            // i saw this from a repo but its amazing 
            // we create a type of functions and store them in an array then apply them one by one
            let fen_parsers_functions: [Fen_parser; 2] = [load_board , load_side_to_move];
            //we create a duplicate because for some reason if not the parsing wont be fine
            let mut new_board = self.clone();
            new_board.reset();

            // now we loop around each parsing function , such a cool thing 
            let mut i : usize = 0;
            while i < 2 && result == Ok(()) {
                let parser = &fen_parsers_functions[i];
                let part = &fen_parts[i];
                let part_parsed_ok = parser(&mut new_board,part);
                result = if part_parsed_ok {
                    Ok(())
                } else {
                    Err(i as u8 +1)
                };
                i += 1;
            }
            
            //if parsing when well we replace the old board wit the new one
            if result == Ok(()) {
                *self = new_board; 
            }
        } 
        result 
        //each function that is used to parse the FEN string is gonna be checked if anything bad
        //happens ... untile i find a rustier way of doing this 
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
        println!("   A B C D E F G H");
        match self.side_to_mode {
            SIDES::white  => print!("White"),
            SIDES::black => print!("black"),

        } 
        println!(" to move");
    }
}
// part 1 : board
fn load_board(board :&mut Chessboard,part:&str) -> bool {
    //we use these to keep track of the square we are in
    let mut rank : u8 = 0;
    let mut file : u8 = 0;
    // i use this to handle errors 
    let mut result_code = true;

    for c in part.chars(){
        let square = rank*8 + file;
        match c {
            'P' =>  set_bit!(board.bitboards[Pieces::P],square), 
            'R' =>  set_bit!(board.bitboards[Pieces::R],square), 
            'N' =>  set_bit!(board.bitboards[Pieces::N],square), 
            'B' =>  set_bit!(board.bitboards[Pieces::B],square), 
            'Q' =>  set_bit!(board.bitboards[Pieces::Q],square), 
            'K' =>  set_bit!(board.bitboards[Pieces::K],square), 
            'p' =>  set_bit!(board.bitboards[Pieces::p],square),
            'r' =>  set_bit!(board.bitboards[Pieces::r],square), 
            'n' =>  set_bit!(board.bitboards[Pieces::n],square), 
            'b' =>  set_bit!(board.bitboards[Pieces::b],square), 
            'q' =>  set_bit!(board.bitboards[Pieces::q],square), 
            'k' =>  set_bit!(board.bitboards[Pieces::k],square), 
            
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
//part 2 : parsing the side to move
fn load_side_to_move(board :&mut Chessboard,part:&str) -> bool{
    let letter = part.chars().next().unwrap(); 
    board.side_to_mode = match letter {
        'w' => SIDES::white,
        'b' => SIDES::black,
        _ => return false,
    };
    return true
} 

fn main() {
    let mut chess = Chessboard::new();
    chess.parse_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
    chess.print_chessboard();   
    chess.parse_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9");
    chess.print_chessboard();

}
