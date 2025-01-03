use crate::Chessboard;
use crate::chessboard::attacks::AttackMasks;
use crate::chessboard::defs::{Pieces,FenPositions,algb_to_square,SQUARE,SQUARE_NAME};

use crate::movegen::MoveGenerator;
use crate::movegen::movecode::{Move};
use crate::movegen::perft::{perft_driver};
use crate::{
    get_move_dst,
    get_move_src,
    get_move_promotion,
};
use crate::MoveMask;
use crate::move_type::ALL_MOVES;


use crate::search::{Search};
/*
* Idk if creating a new instance of Generator a good idea for each handler function , for now let
* it be like this , if problems arouse then well fix it
*/
use std::time::Instant;

// Every line should end in a '\n'
pub fn show_engine_info() {
    println!("id name rustmate");
    println!("id author devpsiarch");
    // We could have more above : option name [thingy]
    // We have to end with uciok
    println!("uciok");
}
//here is the function that parses the move gotten from the UCI and also be made
// This also ignores invalid moves
pub fn parse_move(generator:&mut MoveGenerator,move_str:&str) -> Move{
    // Moves from the UCI look like this : e2e4 , e7e8q
    if move_str.len() == 4 || move_str.len() == 5 {
        // defining these in case we have a promotion
        let mut promoted:&str = "0";
        if move_str.len() == 5 {
            promoted = move_str.get(4..5).unwrap();
        }
        let mut src:u8 = SQUARE::NO_SQUARE;
        let mut dst:u8 = SQUARE::NO_SQUARE;
        let result = algb_to_square(move_str.get(0..2).unwrap());
        match result {
            Some(x) => {
                src = x;
            }
            _ => (),
        }
        let result = algb_to_square(move_str.get(2..4).unwrap());
        match result {
            Some(x) => {
                dst = x;
            }
            _ => (),
        }
        // Check for if the src and the dst are valid or not
        if src == SQUARE::NO_SQUARE || dst == SQUARE::NO_SQUARE {
            // again we ignore
            return 0;
        }
        // Now we generate the moves for this pos and make the move
        for i in 0..generator.moves.count {
            let mov = generator.moves.list[i];
            // We found our move
            if src == get_move_src!(mov) as u8 && dst == get_move_dst!(mov) as u8 {
                // check if the promotion also matches
                let p = get_move_promotion!(mov) as usize;
                if p != Pieces::NONE as usize {
                    if (p == Pieces::Q || p == Pieces::q) && promoted == "q" {
                        return mov;
                    }
                    else if (p == Pieces::B || p == Pieces::b) && promoted == "b" {
                        return mov;
                    }
                    else if (p == Pieces::N || p == Pieces::n) && promoted == "n" {
                        return mov;
                    }
                    else if (p == Pieces::R || p == Pieces::r) && promoted == "r" {
                        return mov;
                    }
                    // We go again
                    continue;
                }
                return mov;
            }
        }
    }
    0
}
// Here we define the "position" function Handler
// it returns a bool for the game state , either he game as ended or not
pub fn position_handler(board:&mut Chessboard,atk:&AttackMasks,parts:&Vec<&str>) -> bool {
    let mut moves_index :usize = 0;
    match parts[1] {
        // We just init the start position then read the move
        "startpos" => {
            board.init_board(FenPositions::STARTING_POSITION);
            // getting if there is moves after that
            if let Some(_state) = parts.get(2) {        // checking if parts[2] exists
                moves_index = 2; 
            }else{                  // else we are done here and return
                () 
            }
        } 
        "fen" => {
            // Checking if a fen is here that the engine can parse (idk if the uci can get diff fens)
            if let Some(_state) = parts.get(2..8) {
                // We collect the parts of them back in a fen 
                let fen_parts = &parts[2..8];
                let fen = fen_parts.join(" ");
                board.init_board(&fen);         // idk if its good idea that this panics
                if let Some(_state) = parts.get(8) {        // checking if parts[2] exists
                    moves_index = 8; 
                }else{                  // else we are done here and return
                    () 
                }
            }else{
                () 
            }
        }
        // ignore the invalid command
        _ => (),
    }
    
    // Now here we handle the moves and make them on the board
    // First we check if the "moves_index" is accualy pointing to a "moves" string
    if parts[moves_index] != "moves" {
        // might ness up the UCI
        //board.print_chessboard();
        return true;
    }
    // else we get the moves in a vector and make them if they are : availble and legal 
    let moves_set = &parts[(moves_index+1)..];
    // I create it here because that is faster then making a new one each move
    let mut generator = MoveGenerator::new(board,&atk);
    for mov in moves_set {
        // mov.parse().decode().seach_in_generator() if found => make it : ignore it
        // println!("move is {mov}");
        generator.generate_moves();
        if generator.check_mate() || generator.stale_mate() {
            // this indectes that the game has ended
            return false;
        }
        let mv = parse_move(&mut generator,mov);
        // Getting the move failed for some reason , we dont care
        if mv != 0 {
            generator.make_move(mv,ALL_MOVES);
        }
    }
    //board.print_chessboard();
    true
}

// a function converts a Move to a uci move
pub fn get_uci_move(mov:Move) -> String {
    let mut result = String::new();
    result.push_str(SQUARE_NAME[get_move_src!(mov) as usize]);
    result.push_str(SQUARE_NAME[get_move_dst!(mov) as usize]);
    let promo = get_move_promotion!(mov) as usize;
    match promo {
        Pieces::Q | Pieces::q => result.push_str("q"), 
        Pieces::B | Pieces::b => result.push_str("b"), 
        Pieces::N | Pieces::n => result.push_str("n"), 
        Pieces::R | Pieces::r => result.push_str("r"),
        _ => (),
    }
    result
} 

// handler for the "go" command , this will be edited to handler more commands in the future
pub fn go_handler(board:&mut Chessboard,atk:&AttackMasks,parts:&Vec<&str>) {
    // some "go" commands : go depth 5 , go infinte , go nodes 10000 ... lets try to make it easy
    // for future me to add em
    
    // Handler the go depth and go nodes
    if parts.len() == 3  {
        match parts[1] {
            "depth" => {
                // getting the depth
                let depth:u32 = parts[2].parse().unwrap();
                let search_result = Search::search_move(board,atk,depth);
                // Here i wanted move controle over if something went wrong
                match search_result {
                    Some(mv) => println!("bestmove {}",get_uci_move(mv)),
                    _ => println!("Error while searching for move , check 'go_handler'"),
                }
            }
            "perft" => {
                let depth:u32 = parts[2].parse().unwrap();
                let start = Instant::now(); 
                println!("Moves found: {}",perft_driver(board,atk, depth));
                println!("Time taken: {:.2?}",start.elapsed());
            }
            "nodes" => {
                todo!();
            }
            _ => return,
        }
    }
    if parts.len() == 2  {
        todo!()
    }
}
