use super::defs::{SIDES,Pieces,Castle,UNICODE_PIECES,SQUARE_NAME,MAX_GAME_MOVES,MAX_MOVE_RULE};

//idk why the compiler told me to explicitly use the macros here but hey it works
use crate::get_bit;
use crate::set_bit;
use crate::Chessboard;
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



// here zwe will implement only the parsing fen methode
impl Chessboard {
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
            let fen_parsers_functions: [Fen_parser; LEN_FEN_STRING] = [
                 load_board ,
                 load_side_to_move ,
                 load_castling_rights ,
                 load_en_passant,
                 load_half_move_clock,
                 load_move_count   
            ];
            //we create a duplicate because for some reason if not the parsing wont be fine
            let mut new_board = self.clone();
            new_board.reset();

            // now we loop around each parsing function , such a cool thing 
            let mut i : usize = 0;
            while i < LEN_FEN_STRING && result == Ok(()) {
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
        // here we print the board pieces
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
        // sides here duh 
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
        //here we print the en passant thingy
        let square : usize = self.en_passant as usize;
        println!("en passant on : {}",SQUARE_NAME[square]);
        //printing the half move clock
        println!("half_move_clock : {}",self.half_move_clock);
        //printing the current moves played
        println!("moves played : {}",self.move_count);
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

//this is a helper function that returns the correct square number for each algebriac square 
fn algb_to_square(square:&str) -> Option<u8> {
    match square {
        "a1" => Some(0), "b1" => Some(1), "c1" => Some(2), "d1" => Some(3),
        "e1" => Some(4), "f1" => Some(5), "g1" => Some(6), "h1" => Some(7),
        "a2" => Some(8), "b2" => Some(9), "c2" => Some(10), "d2" => Some(11),
        "e2" => Some(12), "f2" => Some(13), "g2" => Some(14), "h2" => Some(15),
        "a3" => Some(16), "b3" => Some(17), "c3" => Some(18), "d3" => Some(19),
        "e3" => Some(20), "f3" => Some(21), "g3" => Some(22), "h3" => Some(23),
        "a4" => Some(24), "b4" => Some(25), "c4" => Some(26), "d4" => Some(27),
        "e4" => Some(28), "f4" => Some(29), "g4" => Some(30), "h4" => Some(31),
        "a5" => Some(32), "b5" => Some(33), "c5" => Some(34), "d5" => Some(35),
        "e5" => Some(36), "f5" => Some(37), "g5" => Some(38), "h5" => Some(39),
        "a6" => Some(40), "b6" => Some(41), "c6" => Some(42), "d6" => Some(43),
        "e6" => Some(44), "f6" => Some(45), "g6" => Some(46), "h6" => Some(47),
        "a7" => Some(48), "b7" => Some(49), "c7" => Some(50), "d7" => Some(51),
        "e7" => Some(52), "f7" => Some(53), "g7" => Some(54), "h7" => Some(55),
        "a8" => Some(56), "b8" => Some(57), "c8" => Some(58), "d8" => Some(59),
        "e8" => Some(60), "f8" => Some(61), "g8" => Some(62), "h8" => Some(63),
        _ => None,  // Return None for invalid input
    }
}

// part 4 ; parsing the en passant square
fn load_en_passant(board :&mut Chessboard,part:&str) -> bool {
    let length = part.len();
    let mut char_ok = 0;

    // checking if the part is just a dash which is fine 
    if length == 1 {
        let Some(x) = part.chars().next() else {todo!()};   // this is the only way that it would
        // work so ...
        if x == DASH {
            char_ok += 1
        }
    }
    //checking if there is acually a square that allows en passant 
    if length == 2 {
        let square = algb_to_square(part);
        match square {
            Some(x) => {
                board.en_passant = x;
                char_ok += 2;
            }
            None => (),
        }
    }
    (char_ok == 1 || char_ok == 2) && (length == char_ok)
}

//part 5 : half move clock , idk what does this do tbh but guess its inportant 
fn load_half_move_clock(board :&mut Chessboard,part:&str) -> bool {
    let length = part.len();
    let mut result = false;
    
    //we have to check either the number is of a degit or two 
    if length == 1 || length == 2 {
        let Ok(x) = part.parse::<u8>() else {todo!()};
        if x < MAX_MOVE_RULE {
            board.half_move_clock = x;
            result = true;
        } 
    }
    result
}

//part 6 : baciclly the current move of the game ,
//i assumed the max nunber to be 2048 idk how accurate that is.
fn load_move_count(board :&mut Chessboard,part:&str) -> bool {
    let length = part.len();
    let mut result = false;
    
    //we have to check either the number is of a degit or two 
    if length == 1 || length == 4 {
        let Ok(x) = part.parse::<u16>() else {todo!()};
        if x < MAX_GAME_MOVES {
            board.move_count = x;
            result = true;
        } 
    }
    result
}
