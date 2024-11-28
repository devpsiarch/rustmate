mod defs;
use defs::{SIDES,Pieces,Castle,UNICODE_PIECES};
mod bitboard;
use bitboard::{Bitboard};

//idk why the compiler told me to explicitly use the macros here but hey it works
use crate::get_bit;
use crate::set_bit;
/*mod defs;
use super::defs::{SIDES,Pieces,UNICODE_PIECES};
mod bitboard;
use super::bitboard::{Bitboard};*/


//these are used exclusivly for fen parsing
const SPLITTER: char = '/';
const DASH: char = '-';
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const LIST_OF_CASTLE: &str = "KQkq-";
const LEN_FEN_STRING: usize = 6;

#[allow(non_camel_case_types)]
type Fen_result = Result<(),u8>;
#[allow(non_camel_case_types)]
type Fen_parser = fn(board:&mut Chessboard,part:&str) -> bool;


#[derive(Clone)]
pub struct Chessboard {
    bitboards : [Bitboard;12],
    side_to_mode : SIDES,
    castling_rights :u8,
}
impl Chessboard {
    //We might have to alwasys go back here too more features as the programs grows
    //TODO: UI , 
    pub fn new() -> Self {
        Self {
            bitboards : [0;12], 
            side_to_mode : SIDES::white,
            castling_rights : 0,
        } 
    }
    //this may have to be set to private later on
    pub fn reset(&mut self) {
        self.bitboards = [0;12];
        self.side_to_mode = SIDES::white;
        self.castling_rights = 0;
    } 
    pub fn parse_fen(&mut self,fen:&str) -> Fen_result {
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
            let fen_parsers_functions: [Fen_parser; 3] = [
                 load_board ,
                 load_side_to_move ,
                 load_castling_rights
            ];
            //we create a duplicate because for some reason if not the parsing wont be fine
            let mut new_board = self.clone();
            new_board.reset();

            // now we loop around each parsing function , such a cool thing 
            let mut i : usize = 0;
            while i < 3 && result == Ok(()) {
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
    pub fn print_chessboard(&self){
        //this assumes that the input Chessboard is correct such that no 2 Pieces occupy the same
        for r in 0..8 {
            for c in 0..8 {
                if c == 0 {
                    print!("{} |",8-r);
                }
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
        //decoding castling rigths
        print!("Castling rights : ");
        println!("{}{}{}{}",
            if get_bit!(self.castling_rights,3) == 1 {'K'} else {DASH},
            if get_bit!(self.castling_rights,2) == 1 {'Q'} else {DASH},
            if get_bit!(self.castling_rights,1) == 1 {'k'} else {DASH},
            if get_bit!(self.castling_rights,0) == 1 {'q'} else {DASH},
        ); 
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
//part 3 : parsing the castling rights
fn load_castling_rights(board :&mut Chessboard,part:&str) -> bool {
    //aperently , castling can be of length 1 if no players has castling rights 
    let length = part.len();
    let mut char_ok = 0;

    if (1..=4).contains(&length) {
        for c in part.chars() {
            if LIST_OF_CASTLE.contains(c) {
                char_ok += 1;
                match c {
                    'K' => board.castling_rights |= Castle::K, 
                    'Q' => board.castling_rights |= Castle::Q, 
                    'k' => board.castling_rights |= Castle::k, 
                    'q' => board.castling_rights |= Castle::q, 
                    _ => (),
                }
            }

        }
    }

    (length >= 1) && (length == char_ok)
}

